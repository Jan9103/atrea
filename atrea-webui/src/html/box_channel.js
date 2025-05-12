import {x,g} from "./libs/xeact.js";
import{cr,ce,n,send_msg}from"./atrea.js";

const open_channel=(click_event)=>{
  let channel=click_event.target.innerText.toLowerCase().trim();
  send_msg("view_channel",{
    "name": channel,
    "login": channel
  });
};

const fill_raid_table=(table,data,ir)=>{
  data.forEach((co)=>{
    var tr=n("tr");

    var td=n("td");
    var btn=n("button");
    btn.innerText=(ir?co["raider"]:co["target"]);
    btn.onclick=open_channel;
    td.appendChild(btn);
    tr.appendChild(td);

    td=n("td");td.innerText=co["raid_count"];tr.appendChild(td);
    td=n("td");td.innerText=co["total_viewers"];tr.appendChild(td);
    td=n("td");td.innerText=co["average_raid_size"];tr.appendChild(td);
    table.appendChild(tr);
  });
};
const fill_so_table=(table,data,ir)=>{
  data.forEach((co)=>{
    var tr=n("tr");

    var td=n("td");
    var btn=n("button");
    btn.innerText=(ir?co["author"]:co["target"]);
    btn.onclick=open_channel;
    td.appendChild(btn);
    tr.appendChild(td);

    td=n("td");td.innerText=co["shoutout_count"];tr.appendChild(td);
    table.appendChild(tr);
  });
};

var params = new URLSearchParams(window.location.search);
var channel = params.get("login");

fetch("api/channel/"+channel+"/twitch_info")
  .then(cr)
  .then(response=>response.json())
  .then(res=>{
    g("cvi_img").src=res["profile_image_url"];
    g("cvi_title").innerText=res["display_name"];
    g("cvi_twitch_link").href="https://twitch.tv/"+res["login"];
    g("cvi_desc").innerText=res["description"];
    g("cvi_id").innerText=res["id"];
    var created_date = new Date(res["created_at"] * 1_000);
    g("cvi_created").innerText=created_date.toISOString().replace('T', ' ').slice(0, -5);
    g("cvi_b_type").innerText=res["broadcaster_type"];
  })
  .catch(ce);

fetch("api/raids/to/"+channel+"/stats")
  .then(cr)
  .then(response=>response.json())
  .then(incomming_raiders=>fill_raid_table(g("cvi_raids_i"),incomming_raiders,true))
  .catch(ce);
fetch("api/raids/from/"+channel+"/stats")
  .then(cr)
  .then(response=>response.json())
  .then(incomming_raiders=>fill_raid_table(g("cvi_raids_o"),incomming_raiders,false))
  .catch(ce);

fetch("api/shoutouts/to/"+channel+"/stats")
  .then(cr)
  .then(response=>response.json())
  .then(incomming_so=>fill_so_table(g("cvi_so_i"),incomming_so,true))
  .catch(ce);
fetch("api/shoutouts/from/"+channel+"/stats")
  .then(cr)
  .then(response=>response.json())
  .then(incomming_so=>fill_so_table(g("cvi_so_o"),incomming_so,false))
  .catch(ce);

fetch("api/channel/"+channel+"/known_viewers")
  .then(cr)
  .then(response=>response.json())
  .then((viewers)=>{
    let table=g("cvi_known_viewers");
    viewers.forEach((vo)=>{
      var tr=n("tr");
      var td=n("td");td.innerText=vo["viewer"];tr.appendChild(td);
      td=n("td");td.innerText=vo["score"];tr.appendChild(td);
      table.appendChild(tr);
    });
  })
  .catch(ce);
  
const render_neighbour_graph=()=>{
  x(g("cvi_raid_neighbour_graph"));
  fetch("api/channel/"+channel+"/force_graph_neighbours")
    .then(cr)
    .then(response=>response.json())
    .then((gData)=>{
      gData={
        links:gData["links"],
        nodes:gData["nodes"].map(n=>{
          const img=new Image();
          img.src="api/channel/"+n["id"]+"/image";
          n["img"]=img;
          //n["color"]=n["val"]==5?"#f00":(n["val"]==3?"#aa0":"#0f0");
          return n;
        }),
      };
      let ngc=g("cvi_raid_neighbour_graph");
      const graph=new ForceGraph()
        (ngc)
          .graphData(gData)
          .width(ngc.offsetWidth)
          .height(ngc.offsetHeight)
          .backgroundColor("#425")
          .nodeCanvasObject(({ img, x, y }, ctx) => {
            const size = 15;
            // cant catch error here since it otherwise creates infinite errors
            // and cant catch it outside the closure since it dosn't get caught then..
            ctx.drawImage(img, x - size / 2, y - size / 2, size, size);
          })
          .nodePointerAreaPaint((node, color, ctx) => {
            const size = 15;
            ctx.fillStyle = color;
            ctx.fillRect(node.x - size / 2, node.y - size / 2, size, size);
          })
          .linkColor(()=>"#088")
          .onNodeClick((node) => {
            console.log(node);
            window.top.postMessage(JSON.stringify({
              "action": "view_channel",
              "name": node["name"],
              "login": node["id"],
            }));
          })
          ;
    })
    .catch(ce);
}

g("cvi_raid_neighbour_graph_btn").onclick=render_neighbour_graph;

import {g} from "./libs/xeact.js";
const n=(e)=>document.createElement(e);

const open_channel=(click_event)=>{
  let channel=click_event.target.innerText.toLowerCase().trim();
  window.top.postMessage(JSON.stringify({
    "action": "view_channel",
    "name": channel,
    "login": channel
  }));
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

var params = new URLSearchParams(window.location.search);
var channel = params.get("login");

fetch("api/channel/"+channel+"/twitch_info")
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
  .catch(error=>console.error('Error:',error));

fetch("api/raids/to/"+channel+"/stats")
  .then(response=>response.json())
  .then(incomming_raiders=>fill_raid_table(g("cvi_raids_i"),incomming_raiders,true))
  .catch(error=>console.error('Error:',error));
fetch("api/raids/from/"+channel+"/stats")
  .then(response=>response.json())
  .then(incomming_raiders=>fill_raid_table(g("cvi_raids_o"),incomming_raiders,false))
  .catch(error=>console.error('Error:',error));

fetch("api/channel/"+channel+"/known_viewers")
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
  .catch(error=>console.error('Error:',error));

import {r,g,x,t,u} from "./xeact.js";
const n=(e)=>document.createElement(e);
const PAGESIZE=50;

const fetch_details=(login,img_node,name_node,desc_node)=>{
  console.log("fetch_details "+login);
  fetch("api/channel/"+login+"/twitch_info")
    .then(response=>response.json())
    .then(res=>{
      console.log("fetch_details->res "+login);
      console.log(res);
      console.log(desc_node);
      img_node.src=res["profile_image_url"];
      var name=res["display_name"];
      if(res["broadcaster_type"]!=""){name=name+" ("+res["broadcaster_type"]+")"}
      name_node.innerText=res["display_name"];
      name_node.onclick = show_channel_popup;
      //desc_node.innerText=(res["description"].length>30?res["description"].substring(0,30)+"..":res["description"]);
      desc_node.innerText=res["description"].split("\n")[0];
      console.log("fetch_details->done "+login);
    })
    .catch(error=>console.error('Error:',error));
};

const append_render=(channels)=>{
  if(channels.length == PAGESIZE){
    g("load_more_button").classList.remove("gone");
  };
  let ul_node=g("recommend_channels");
  for(const channel of channels){
    let li_node=n("div");
    let img_node=n("img");
    img_node.src="loading_image.svg";
    img_node.classList.add("small_pp");
    li_node.appendChild(img_node);
    let div_node=n("div");
    div_node.appendChild(t("["+Math.round(channel["score"])+"] "));
    let a_node=n("a");
    let name_node=n("span");
    name_node.innerText=channel["channel"];
    a_node.appendChild(name_node);
    div_node.appendChild(a_node);
    div_node.appendChild(n("br"));
    let desc_node=n("span");
    div_node.appendChild(desc_node);
    fetch_details(channel["channel"],img_node,name_node,desc_node);
    li_node.appendChild(div_node);
    ul_node.appendChild(li_node);
  }
};

const render_more=()=>{
  g("load_more_button").classList.add("gone");
  let params=new URLSearchParams(window.location.search);
  let alg=params.get("algorithm") || "brta1";
  let offset=g("recommend_channels").childNodes.length;
  fetch("api/recs/general/"+alg+"?limit="+PAGESIZE+"&offset="+offset)
    .then(response=>response.json())
    .then(append_render)
    .catch(error=>console.error('Error:',error));
}

r(()=>{
  g("load_more_button").onclick=render_more;
  render_more();
});

const hide_channel_popup=()=>{
  let cp=g("channel_popup");
  cp.classList.add("gone");
  x(cp);
};

const show_channel_popup=(click_event)=>{
  let channel_name = click_event.target.innerText.toLowerCase().trim();
  let cp=g("channel_popup");
  let cb=n("button");
  cb.innerText = "X";
  cb.onclick = hide_channel_popup;
  cp.appendChild(cb);
  let ifr=n("iframe");
  ifr.src="channel_view_iframe.html?channel="+channel_name;
  cp.appendChild(ifr);
  cp.classList.remove("gone");
}

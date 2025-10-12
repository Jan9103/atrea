import{r,g,t}from"./libs/xeact.js";
import{cr,ce,n}from"./atrea.js";

const render_vod=(parent,vod)=>{
  let div_node=n("div");
  div_node.onclick=()=>{window.open("@!base_url!@/videos/"+vod["id"], "_blank").focus();};

  let img_node=n("img");
  img_node.setAttribute("src","@!base_url!@"+vod["web_thumbnail_path"]);
  div_node.appendChild(img_node);

  let span_node=n("span");
  span_node.innerText=vod["title"];
  div_node.appendChild(span_node);

  parent.appendChild(div_node);
};

const render_vod_category=(parent,title,vods,count)=>{
  if(vods.length==0||count==0){return;}
  vods.sort((a,b)=>{return a["streamed_at"]<b["streamed_at"]?1:-1;});
  let vod_div=n("div");
  var h3=n("h3");
  h3.innerText=title;
  vod_div.appendChild(h3);
  let vod_list_div=n("div");
  vod_list_div.classList.add("ganymede_vod_list_div");
  for(const vod of vods.slice(0,count)){render_vod(vod_list_div,vod);}
  vod_div.appendChild(vod_list_div);
  parent.appendChild(vod_div);
};

const apply_vods=vod_list=>{
  let ganymede_div=n("div"); // wrap in one div to avoid it beeing torn apart by other plugins
  render_vod_category(ganymede_div,"Recent clips (ganymede)",     vod_list.filter(i=>i["type"]=="clip"),     parseInt("@!clip_count!@"));
  render_vod_category(ganymede_div,"Recent archives (ganymede)",  vod_list.filter(i=>i["type"]=="archive"),  parseInt("@!archive_count!@"));
  render_vod_category(ganymede_div,"Recent highlights (ganymede)",vod_list.filter(i=>i["type"]=="highlight"),parseInt("@!highlight_count!@"));
  render_vod_category(ganymede_div,"Recent uploads (ganymede)",   vod_list.filter(i=>i["type"]=="upload"),   parseInt("@!upload_count!@"));
  g("cvi_raids").appendChild(ganymede_div);
};

const apply_1=data=>{
  // add link next to name
  let a_node=n("a");
  a_node.setAttribute("rel","noopener noreferrer");
  a_node.setAttribute("target","_blank");
  a_node.setAttribute("href","@!base_url!@/channels/"+data["name"])
  let img_node=n("img");
  img_node.setAttribute("src","@!base_url!@/images/ganymede_logo.png");
  img_node.setAttribute("height","30px");
  a_node.appendChild(img_node);
  g("cvi_twitch_link").parentNode.appendChild(a_node);

  // get vods
  fetch("@!base_url!@/api/v1/vod?channel_id="+data["id"])
    .then(cr)
    .then(r=>r.json())
    .then(r=>apply_vods(r["data"]))
    .catch(ce);
};

r(()=>{
  let params=new URLSearchParams(window.location.search);
  let channel_login=params.get("login");
  fetch("@!base_url!@/api/v1/channel/name/"+channel_login)
    .then(resp=>{
      if(resp.ok){
        return resp.json();
      }else if(resp.status==404){
        return null;
      }else{
        throw new Error("Not 2xx response from "+resp.url, {"cause": resp});
      }
    })
    .then(res=>{
      if(res!=null){apply_1(res["data"]);}
    })
    .catch(ce);
})

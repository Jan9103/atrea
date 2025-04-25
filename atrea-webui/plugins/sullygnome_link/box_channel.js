// it would be easy to scrape it, but i want to respect the wishes of the website-owner
// https://sullygnome.com/about

import {g,r,t} from "./libs/xeact.js";
const n=(e)=>document.createElement(e);

r(()=>{
  let params=new URLSearchParams(window.location.search);
  let login_name=params.get("login");
  let a_node=n("a");
  a_node.setAttribute("rel", "noopener noreferrer");
  a_node.setAttribute("target", "_blank");
  a_node.setAttribute("href", "https://sullygnome.com/channel/"+login_name);
  let img_node=n("img");
  img_node.setAttribute("src", "https://sullygnome.com/Images/gnome.png");
  img_node.setAttribute("height", "30px");
  a_node.appendChild(img_node);
  g("cvi_twitch_link").parentNode.appendChild(a_node);
});

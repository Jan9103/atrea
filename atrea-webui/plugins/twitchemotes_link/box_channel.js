import {g,r,t} from "./libs/xeact.js";
const n=(e)=>document.createElement(e);

r(()=>{
  let a_node=n("a");
  a_node.onclick=()=>{
    let cid=g("cvi_id").innerText.trim();
    window.open("https://twitchemotes.com/channels/"+cid, "_blank").focus();
  };
  let img_node=n("img");
  img_node.setAttribute("src", "https://twitchemotes.com/favicon.png");
  img_node.setAttribute("height", "30px");
  a_node.appendChild(img_node);
  g("cvi_twitch_link").parentNode.appendChild(a_node);
});

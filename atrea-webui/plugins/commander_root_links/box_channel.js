import {g,r,t} from "./libs/xeact.js";
const n=(e)=>document.createElement(e);

const gen_li_node=(text, url)=>{
  let li_node=n("li")
  let a_node=n("a");
  a_node.innerText=text;
  a_node.onclick=()=>{
    let cid=g("cvi_id").innerText.trim();
    window.open(
      url.replace("[CID]", cid),
      "_blank"
    ).focus();
  };
  li_node.appendChild(a_node);
  return li_node;
};

r(()=>{
  let div_node=n("div");

  let h3=n("h3");
  h3.innerText="Command Root tools";
  div_node.appendChild(h3);
  let ul=n("ul");

  ul.appendChild(gen_li_node(
    "View Emotes and Badges",
    "https://twitch-tools.rootonline.de/emotes.php?channel_id=[CID]"
  ));
  ul.appendChild(gen_li_node(
    "View plus-points",
    "https://twitch-tools.rootonline.de/plusPoints_stats.php?channel_id=[CID]"
  ));

  div_node.appendChild(ul);
  g("cvi_raids").appendChild(div_node);
});

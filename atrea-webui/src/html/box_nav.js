import {r,g,x,t,u} from "./libs/xeact.js";
//import {r,g,x,t,u};
const n=(e)=>document.createElement(e);

const new_box_recs=(alg_name)=>{
  window.top.postMessage(JSON.stringify({
    "action": "show_recs",
    "alg": alg_name,
  }));
};

const render_rec_alg=(ul_node,alg)=>{
    let li_node=n("li");
    let button_node=n("button");
    button_node.innerText=alg["name"];
    button_node.onclick=()=>{new_box_recs(alg["name"]);};
    li_node.appendChild(button_node);
    ul_node.appendChild(li_node);
};

/////// NAV ///////
fetch("api/recs/algorithms/general")
  .then(response=>response.json())
  .then((algos)=>{
    let ul_node=g("nav_rec_alg_inject");
    for(const alg of algos){
      render_rec_alg(ul_node,alg);
    }
  })
  .catch(error=>console.error('Error:',error));

g("nav_rec_help").onclick=()=>{
  window.top.postMessage(JSON.stringify({
    "action": "open_help",
    "site": "recs",
    "title": "Recommendations",
  }));
};

g("nav_credits").onclick=()=>{
  window.top.postMessage(JSON.stringify({
    "action": "open_help",
    "site": "credits",
    "title": "Credits",
  }));
};

g("nav_known_viewers").onclick=()=>{
  window.top.postMessage(JSON.stringify({
    "action": "open_known_viewers"
  }))
};

g("nav_liked_channels").onclick=()=>{
  window.top.postMessage(JSON.stringify({
    "action": "open_liked_channels"
  }));
};

g("nav_rel_graph").onclick=()=>{
  window.top.postMessage(JSON.stringify({
    "action": "show_rel_graph",
  }));
};

g("nav_plugins").onclick=()=>{
  window.top.postMessage(JSON.stringify({
    "action": "show_plugins",
  }));
};

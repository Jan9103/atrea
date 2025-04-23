import {r,g,x,t,u} from "./libs/xeact.js";
//import {r,g,x,t,u};
const n=(e)=>document.createElement(e);

const new_box_recs=(alg_name)=>{
  new WinBox("Recommendations ("+alg_name+")", {
    url: "./box_recs.html?algo="+alg_name,
    background: "#066",
  });
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
  new WinBox("Help: Recommendations", {
    url: "./box_help_recs.html",
    background: "#060",
  });
};

g("nav_credits").onclick=()=>{
  new WinBox("Credits", {
    url: "./box_help_credits.html",
    background: "#060",
  });
};

g("nav_known_viewers").onclick=()=>{
  new WinBox("Known Viewers", {
    url: "./box_known_viewers.html",
    background: "#606",
  });
};

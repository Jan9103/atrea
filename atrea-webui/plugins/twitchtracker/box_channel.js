import {g,r,t} from "./libs/xeact.js";
import{cr,ce}from"./atrea.js";
const n=(e)=>document.createElement(e);

const gen_tr=(h_text,d_text)=>{
  let tr=n("tr");
  let th=n("th");
  th.innerText=h_text;
  tr.appendChild(th);
  let td=n("td");
  td.innerText=d_text;
  tr.appendChild(td);
  return tr;
};

r(()=>{
  var params=new URLSearchParams(window.location.search);
  var login_name=params.get("login");
  
  fetch("https://twitchtracker.com/api/channels/summary/"+login_name)
    .then(cr)
    .then(response=>response.json())
    .then(res=>{
      let div=n("div");
      let h3=n("h3");
      h3.appendChild(t("30 day stats ("));
      let button=n("button");
      button.innerText="twitchtracker";
      button.onclick=()=>{
        window.open(
          "https://twitchtracker.com/"+login_name,
          "_blank"
        ).focus();
      };
      h3.appendChild(button);
      h3.appendChild(t(")"));
      div.appendChild(h3);
      let table=n("table");
      table.appendChild(gen_tr("Live (mins)", res["minutes_streamed"]));
      table.appendChild(gen_tr("Viewers (avg)", res["avg_viewers"]));
      table.appendChild(gen_tr("Viewers (max)", res["max_viewers"]));
      table.appendChild(gen_tr("Watchtime (h)", res["hours_watched"]));
      table.appendChild(gen_tr("New Followers", res["followers"]));
      table.appendChild(gen_tr("Total Followers", res["followers_total"]));
      div.appendChild(table);
      g("cvi_top").appendChild(div);
    })
    .catch(ce);
});

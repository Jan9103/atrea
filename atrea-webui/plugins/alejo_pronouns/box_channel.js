import {g,r,t} from "./libs/xeact.js";
const n=(e)=>document.createElement(e);

const PRONOUN_TABLE = {
  "aeaer": "ae/aer",
  "any": "any",
  "eem": "e/em/eir",
  "faefaer": "fae/faer",
  "hehim": "he/him",
  "heshe": "he/she",
  "hethem": "he/them",
  "itits": "it/its",
  "other": "other",
  "perper": "per/per",
  "sheher": "she/her",
  "shethem": "she/them",
  "theythem": "they/them",
  "vever": "ve/ver",
  "xexem": "xe/xem",
  "ziehir": "zie/hir",
};

r(()=>{
  var params=new URLSearchParams(window.location.search);
  var login_name=params.get("login");

  fetch("https://pronouns.alejo.io/api/users/"+login_name)
    .then(response=>response.json())
    .then(res=>{
      if(res.length==1){
        let pid=res[0]["pronoun_id"];
        let pr=PRONOUN_TABLE[pid] || pid;
        let name_node=g("cvi_title");
        name_node.parentNode.insertBefore(t(" ("+pr+")"), name_node.nextSibling);
      }
    })
    .catch(error=>console.error('Error:',error));
});

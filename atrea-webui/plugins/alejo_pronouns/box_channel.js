import {g,r,t} from "./libs/xeact.js";
import{ce,cr}from"./atrea.js";
const n=(e)=>document.createElement(e);

// cache of https://pronouns.alejo.io/api/pronouns
// since it hasn't changed in years it seems to be reasonable to half the requests by caching this
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
    .then(cr)
    .then(response=>response.json())
    .then(res=>{
      if(res.length==1){
        let pid=res[0]["pronoun_id"];
        let pr=PRONOUN_TABLE[pid] || pid;
        let name_node=g("cvi_title");
        name_node.parentNode.insertBefore(t(" ("+pr+")"), name_node.nextSibling);
      }
    })
    .catch(ce);
});

import{g,r,t}from"./libs/xeact.js";
import{ce,cr}from"./atrea.js";
const n=(e)=>document.createElement(e);

const PRONOUN_TABLE = {
  "any": "any",
  "avoid": "avoid",
  "he": "he/him",
  "it": "it/its",
  "other": "other",
  "she": "she/her",
  "they": "they/them",
};

r(()=>{
  let cid=g("cvi_id").innerText.trim();

  fetch("https://pronoundb.org/api/v2/lookup?platform=twitch&ids="+cid, {
    "headers": {
      // ERROR: Reason: header ‘user-agent’ is not allowed according to header ‘Access-Control-Allow-Headers’ from CORS preflight response
      // "User-Agent": "ATREA webui pronoundb-plugin <https://github.com/Jan9103/atrea/tree/main/atrea-webui/plugins/pronoundb>"
    }
  })
    .then(cr)
    .then(response=>response.json())
    .then(res=>{
      let cids=""+cid;
      let pid=null;
      try{pid=res[cids]["sets"]["en"];}catch{}
      if(pid!=null){
        let pr=PRONOUN_TABLE[pid] || pid;
        let name_node=g("cvi_title");
        name_node.parentNode.insertBefore(t(" ("+pr+")"), name_node.nextSibling);
      }
    })
    .catch(ce);
});

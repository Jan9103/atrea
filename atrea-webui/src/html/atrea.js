const n=(e)=>document.createElement(e);

const send_msg=(action,body)=>{
  let msg=body==undefined?{}:body;
  msg["action"]=action;
  window.top.postMessage(JSON.stringify(msg));
};

const show_error=(err)=>{
  let err_disp=[
    {"_":"h1","t":"ERROR"},
    {"_":"table","c":[
      {"_":"tr","c":[
        {"_":"th","t":"Name"},
        {"_":"td","t":err.name},
      ]},
      {"_":"tr","c":[
        {"_":"th","t":"Source"},
        {"_":"td","t":
          Object.hasOwn(err,"fileName")?err.fileName+":"+err.lineNumber:"(not supported by your browser - use firefox)",
        },
      ]},
      {"_":"tr","c":[
        {"_":"th","t":"Cause"},
        {"_":"td","t":err.cause},
      ]},
    ]},
    {"_":"p","t":err.message},
    {"_":"pre","t":Object.hasOwn(err,"stack")?err.stack:""},
  ];
  send_msg("show_error",{"el":err_disp});
};
const ce=(err)=>{
  show_error(err);
  console.error('Error',err);
};
const cr=async (resp)=>{
  if(!resp.ok){
    throw new Error("Not 2xx response from "+resp.url, {"cause": resp})
  }else{return resp;}
};

export {
  n,
  send_msg,
  show_error,
  ce,
  cr,
};

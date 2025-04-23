window.onmessage=(event)=>{
  let msg=JSON.parse(event.data);
  console.log("MAILBOX: "+event.data);
  switch(msg["action"]){
    case "view_channel":
      new WinBox("Channel: "+msg["name"], {
        url: "./box_channel.html?login="+msg["login"],
        background: "#660",
      });
      break;
    default:
      console.error("ERROR: invalid incomming message: "+event.data);
  };
};

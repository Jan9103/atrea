import{r,g}from"./libs/xeact.js";
import{n}from"./atrea.js";

r(()=>{
  let iframe=n("iframe");
  iframe.src="/box_nav.html";
  iframe.id="nav_on_root_plugin";
  g("root_body").appendChild(iframe);

  // does not work
  //let style_node=n("style");
  //style_node.innerText=":root{--bg:#425;}";
  //iframe.contentWindow.document.head.appendChild(style_node);
});

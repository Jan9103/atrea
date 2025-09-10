import {r,g,x,t,u} from "./libs/xeact.js";
const n=(e)=>document.createElement(e);

const LETTERS='ABCDEFGHIJKLMNOPQRSTUVXYZABCDEFGHIJKLMNOPQRSTUVXYZ'.split('');
const FONT_SIZE=10;

const start_rain=()=>{
  let rb=g("root_body");
  x(rb);
  let canvas=n("canvas");
  canvas.width=window.innerWidth;
  canvas.height=window.innerHeight;
  let columns=canvas.width / FONT_SIZE;
  var drops=[];
  for(var i=0; i<columns; i++){
    drops[i]=1;
  }
};

r(()=>start_rain);

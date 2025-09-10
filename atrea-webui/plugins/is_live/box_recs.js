import{r,g}from"./libs/xeact.js";
import(cr,ce)from"./atrea.js";

const is_live((channel)=>{
  // TODO: between thingy cache
  fetch("https://www.twitch.tv/"+channel)
    .then(cr)
    .then(r=>r.json())
    .then((d)=>{
      return d.includes('isLiveBroadcast');
    })
    .catch(ce);
});

r(()=>{

});

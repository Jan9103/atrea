/*
@licstart
Copyright (c) 2021 Xe (https://christine.website)
Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
@licend
*/
const h=(name,data={},children=[])=>{const result=typeof name=="function"?name(data):Object.assign(document.createElement(name),data);if(!Array.isArray(children)){children = [children];}result.append(...children);return result;};
const t=(text)=>document.createTextNode(text);
const x=(elem)=>{while(elem.lastChild){elem.removeChild(elem.lastChild);}};
const g=(name)=>document.getElementById(name);
const c=(name)=>document.getElementsByClassName(name);
const n=(name)=>document.getElementsByName(name);
const s=(selector)=>Array.from(document.querySelectorAll(selector));
const u=(url="",params={})=>{let result=new URL(url,window.location.href);Object.entries(params).forEach((kv)=>{let [k,v]=kv;result.searchParams.set(k,v);});return result.toString();};
const r=(callback)=>window.addEventListener("DOMContentLoaded",callback);
const useState=(value=undefined)=>{return [()=>value,(x)=>{value=x;}];};
const d=(ms)=>{let debounceTimer=null;return(f)=>{clearTimeout(debounceTimer);debounceTimer=setTimeout(f,ms);};};
export {h,t,x,g,c,n,u,s,r,useState,d};

var s=async(...[t])=>{const a=Object.keys(t).map(Number).sort((e,r)=>e-r),n=new Uint8Array(a.length);return a.forEach((e,r)=>{n[r]=t[e]??0}),n};export{s as default};

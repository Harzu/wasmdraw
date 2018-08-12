document.addEventListener('DOMContentLoaded', async e => {
  const res = await require('./lib.rs')
  res.Init(800, 600)
})

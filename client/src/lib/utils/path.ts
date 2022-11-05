import random from 'seedrandom'

export const generateLeafPath = (scale: number, seed: string) => {
  const rnd = random(seed).double()
  
  const points = []
  
  points.push({x: rnd - 10,  y: rnd + 0.25})
  points.push({x: rnd + 0.25, y: rnd + 7})
  points.push({x: rnd + 10,   y: rnd + 0.25})
  points.push({x: rnd + 5,    y: rnd - 7 })


  return points.map(p => `${p.x*scale},${p.y*scale}`).join(" ")
}
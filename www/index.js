import { Universe, Cell } from 'wasm-game-of-life'
import { memory } from 'wasm-game-of-life/wasm_game_of_life_bg'

const cellSize = 5
const gridColor = '#CCCCCC'
const aliveColor = '#83A598'
const deadColor = '#282828'

const universe = Universe.new(Math.floor(window.innerWidth / cellSize), Math.floor(window.innerHeight / cellSize))
const width = universe.width()
const height = universe.height()

const canvas = document.getElementById('game-of-life-canvas')
canvas.height = cellSize * height
canvas.width = cellSize * width

const ctx = canvas.getContext('2d')

let prevRow = 0
let prevCol = 0

canvas.addEventListener('mousemove', (event) => {
  const boundingRect = canvas.getBoundingClientRect()

  const scaleX = canvas.width / boundingRect.width
  const scaleY = canvas.height / boundingRect.height

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX
  const canvasTop = (event.clientY - boundingRect.top) * scaleY

  const row = Math.floor(canvasTop / cellSize)
  const col = Math.floor(canvasLeft / cellSize)

  if (row === prevRow && col === prevCol) return

  prevRow = row
  prevCol = col

  universe.toggle_cell(row, col)
})

const drawGrid = () => {
  ctx.beginPath()
  ctx.strokeStyle = gridColor

  // vertical lines
  for (let i = 0; i <= width; i ++) {
    ctx.moveTo(i * (cellSize + 1) + 1, 0)
    ctx.lineTo(i * (cellSize + 1) + 1, (cellSize + 1) * height + 1)
  }

  // horizontal lines
  for (let j = 0; j <= height; j ++) {
    ctx.moveTo(0,                           j * (cellSize + 1) + 1)
    ctx.lineTo((cellSize + 1) * width + 1, j * (cellSize + 1) + 1)
  }
  ctx.stroke()
}

const getIndex = (row, col) => {
  return row * width + col
}

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8)
  const mask = 1 << (n % 8)
  return (arr[byte] & mask) === mask
}

const drawCells = () => {
  const cellsPtr = universe.cells()
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8)

  ctx.beginPath()

  for (let row = 0;row < height; row++) {
    for (let col = 0; col < width; col++) {
      const id = getIndex(row, col)

      ctx.fillStyle = bitIsSet(id, cells)
        ? aliveColor
        : deadColor

      ctx.fillRect(
        col * cellSize,
        row * cellSize,
        cellSize,
        cellSize
      )
    }
  }
  ctx.stroke()
}
const renderLoop = () => {
  universe.tick()

  drawCells()

  requestAnimationFrame(renderLoop)
}

requestAnimationFrame(renderLoop)

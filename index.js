const canvas = document.getElementById("board");
const ctx = canvas.getContext("2d");
const timer = document.getElementById("time");

let keysPressed = [];
window.onblur = window.onfocus = window.onfocusout = window.onvisibilitychange = _ => keysPressed = [];
window.onkeydown = e => {
    if (!keysPressed.find(x => x.code == e.code))
        keysPressed.push({ code: e.code, active1: false, active2: false, t: performance.now(), lastT: performance.now() });
}
window.onkeyup = e => keysPressed.splice(keysPressed.findIndex(x => x.code == e.code), 1);

function playGame(garbCols, startQueue, maxPieces) {
    let queue = startQueue || [];
    let hold = { canHold: true, piece: undefined };

    if (!startQueue) appendQueue(queue);

    const board = new Board(new PieceLocation(queue.shift(), [4, 18], 0));

    for (const col of garbCols) {
        board.pushGarbage(col);
    }

    let lastRender = 0;
    let done = 0; // 0 = not done, -1 = fail, 1 = success
    let placedPieces = 0;
    function loop(t) {
        timer.textContent = `Time: ${(performance.now() / 1000).toFixed(2)}`
        if (done) {
            timer.style.color = done == 1 ? "#3fc600" : "#ff0000"
            return;
        }
        if (t - lastRender >= 1000 / 60) {
            let horizontalIdx = keysPressed.findLastIndex(x => vKey("moveLeft", x.code) || vKey("moveRight", x.code));
            if (horizontalIdx > -1) {
                let key = keysPressed[horizontalIdx];
                let dir = vKey("moveLeft", key.code) ? -1 : 1;
                if (!key.active1) {
                    board.moveX(dir);
                    key.active1 = true;
                }

                if (!key.active2 && t - key.t > handl("das")) {
                    key.active2 = true;
                    key.lastT = t;
                }

                if (t - key.t > handl("das") && t - key.lastT > handl("arr")) {
                    board.moveX(dir * (handl("arr") ? Math.min(10, Math.floor((t - key.lastT) / handl("arr"))) : 10));
                    key.lastT = t;
                }
            }

            let verticalIdx = keysPressed.findLastIndex(x => vKey("softDrop", x.code) || vKey("hardDrop", x.code));
            if (verticalIdx > -1) {
                let key = keysPressed[verticalIdx];
                if (!key.pressed) {
                    board.moveY(-(handl("sdr") && key.code == "ArrowDown" ? Math.min(40, Math.floor((t - key.lastT) / handl("sdr"))) : 40));
                    if (vKey("hardDrop", key.code)) {
                        board.placeMinos();
                        placedPieces++;
                        if (!queue.length || placedPieces > maxPieces) {
                            done = -1; // this might be changed later
                        }
                        board.currentPiece = queue.length ? new PieceLocation(queue.shift(), [4, 18], 0) : undefined;
                        key.pressed = true;
                        if (queue.length < 7) if (!startQueue) appendQueue(queue);
                        hold.canHold = true;
                        for (let y = 18; y < 40; y++) {
                            if (board.dta[y].some(c => c != 0)) {
                                done = -1;
                                break;
                            }
                        }
                    }
                }
            }

            let keyCW = keysPressed.findLast(x => vKey("rotateCW", x.code));
            if (keyCW && !keyCW.pressed) {
                keyCW.pressed = true;
                board.rotate(1);
            }

            let keyCCW = keysPressed.findLast(x => vKey("rotateCCW", x.code));
            if (keyCCW && !keyCCW.pressed) {
                keyCCW.pressed = true;
                board.rotate(-1);
            }

            let key180 = keysPressed.findLast(x => vKey("rotate180", x.code));
            if (key180 && !key180.pressed) {
                key180.pressed = true;
                board.rotate(2);
            }

            let keyHold = keysPressed.findLast(x => vKey("hold", x.code));
            if (keyHold && !keyHold.pressed && hold.canHold) {
                keyHold.pressed = true;
                let oldHoldPiece = hold.piece;
                hold = { canHold: false, piece: board.currentPiece.piece };
                let p = oldHoldPiece || queue.shift();
                if (!p) done = -1;
                board.currentPiece = p ? new PieceLocation(p, [4, 18], 0) : undefined;
            }

            if (garbCols && !board.dta.some(r => r.some(c => c == -1))) done = 1;

            ctx.fillStyle = "#2a2a2a";
            ctx.fillRect(0, 0, 1000, 1000);
            if (done) board.dta = board.dta.map(r => r.map(q => -!!q));
            board.draw(ctx);
            for (let i = 0; i < 5; i++) {
                if (!queue[i]) break;
                ctx.fillStyle = PIECES[queue[i]].color;
                for (const mino of PIECES[queue[i]].minos) {
                    ctx.fillRect(550 + 30 * mino[0], 60 + 100 * i - 30 * mino[1], 30, 30);
                }
            }
            if (hold.piece) {
                ctx.fillStyle = PIECES[hold.piece].color;
                for (const mino of PIECES[hold.piece].minos) {
                    ctx.fillRect(50 + 30 * mino[0], 60 - 30 * mino[1], 30, 30);
                }
            }
        }
        requestAnimationFrame(loop);
    }

    requestAnimationFrame(loop);
}

let randomMap = maps[Math.floor(Math.random() * maps.length)];
randomMap.games = [];
let game = new Board(new PieceLocation("Z", [4, 19], 0));
for (const col of randomMap.garb_cols) {
    game.pushGarbage(col);
}
for (const move of randomMap.locs) {
    game.currentPiece = new PieceLocation(move.piece, [move.x, move.y], ["North", "East", "South", "West"].indexOf(move.rotation));
    randomMap.games.push(new Board(game.currentPiece.clone(), structuredClone(game.dta)));
    game.placeMinos();
}
playGame(randomMap.garb_cols, randomMap.queue);

const solutionCanvas = document.getElementById("solution_visual");
const solutionCtx = solutionCanvas.getContext("2d");
let iii = 0;

const showSolutionButton = document.getElementById("show_solution");
const solutionDiv = document.getElementById("solution_div");
const leftButton = document.getElementById("left");
const rightButton = document.getElementById("right");

showSolutionButton.onclick = _ => {
    solutionDiv.style.display = "block";
    randomMap.games[iii].draw(solutionCtx);
}
leftButton.onclick = _ => {
    iii = Math.max(iii - 1, 0);
    randomMap.games[iii].draw(solutionCtx);
}
rightButton.onclick = _ => {
    iii = Math.min(iii + 1, randomMap.games.length - 1);
    randomMap.games[iii].draw(solutionCtx);
}
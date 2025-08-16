const canvas = document.getElementById("board");
const ctx = canvas.getContext("2d");

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
    let done = false;
    let placedPieces = 0;
    function loop(t) {
        if (done) {
            return;
        }
        if (t - lastRender >= 1000 / 60) {
            let horizontalIdx = keysPressed.findLastIndex(x => x.code == "ArrowLeft" || x.code == "ArrowRight");
            if (horizontalIdx > -1) {
                let key = keysPressed[horizontalIdx];
                let dir = key.code == "ArrowLeft" ? -1 : 1;
                if (!key.active1) {
                    board.moveX(dir);
                    key.active1 = true;
                }

                if (!key.active2 && t - key.t > CONFIG.das) {
                    key.active2 = true;
                    key.lastT = t;
                }

                if (t - key.t > CONFIG.das && t - key.lastT > CONFIG.arr) {
                    board.moveX(dir * (CONFIG.arr ? Math.min(10, Math.floor((t - key.lastT) / CONFIG.arr)) : 10));
                    key.lastT = t;
                }
            }

            let verticalIdx = keysPressed.findLastIndex(x => x.code == "ArrowDown" || x.code == "Space");
            if (verticalIdx > -1) {
                let key = keysPressed[verticalIdx];
                if (!key.pressed) {
                    board.moveY(-(CONFIG.sdr && key.code == "ArrowDown" ? Math.min(40, Math.floor((t - key.lastT) / CONFIG.sdr)) : 40));
                    if (key.code == "Space") {
                        board.placeMinos();
                        placedPieces++;
                        if (!queue.length || placedPieces > maxPieces) {
                            done = true;
                        }
                        board.currentPiece = queue.length ? new PieceLocation(queue.shift(), [4, 18], 0) : undefined;
                        key.pressed = true;
                        if (queue.length < 7) if (!startQueue) appendQueue(queue);
                        hold.canHold = true;
                        for (let y = 18; y < 40; y++) {
                            if (board.dta[y].some(c => c != 0)) {
                                done = true;
                                break;
                            }
                        }
                    }
                }
            }

            let keyCW = keysPressed.findLast(x => x.code == "ArrowUp");
            if (keyCW && !keyCW.pressed) {
                keyCW.pressed = true;
                board.rotate(1);
            }

            let keyCCW = keysPressed.findLast(x => x.code == "KeyZ");
            if (keyCCW && !keyCCW.pressed) {
                keyCCW.pressed = true;
                board.rotate(-1);
            }

            let key180 = keysPressed.findLast(x => x.code == "KeyA");
            if (key180 && !key180.pressed) {
                key180.pressed = true;
                board.rotate(2);
            }

            let keyHold = keysPressed.findLast(x => x.code == "KeyC");
            if (keyHold && !keyHold.pressed && hold.canHold) {
                keyHold.pressed = true;
                let oldHoldPiece = hold.piece;
                hold = { canHold: false, piece: board.currentPiece.piece };
                let p = oldHoldPiece || queue.shift();
                if (!p) done = true;
                board.currentPiece = p ? new PieceLocation(p, [4, 18], 0) : undefined;
            }

            if (garbCols && !board.dta.some(r => r.some(c => c == -1))) done = true;

            ctx.fillStyle = "#e9e9e9";
            ctx.fillRect(0, 0, 1000, 1000);
            if (done) board.dta = board.dta.map(r => r.map(q => -!!q));
            board.draw(ctx);
            for (let i = 0; i < 5; i++) {
                if (!queue[i]) break;
                ctx.fillStyle = PIECES[queue[i]].color;
                for (const mino of PIECES[queue[i]].minos) {
                    ctx.fillRect(550 + 30 * mino[0], 30 + 100 * i - 30 * mino[1], 30, 30);
                }
            }
            if (hold.piece) {
                ctx.fillStyle = PIECES[hold.piece].color;
                for (const mino of PIECES[hold.piece].minos) {
                    ctx.fillRect(50 + 30 * mino[0], 30 - 30 * mino[1], 30, 30);
                }
            }
        }
        requestAnimationFrame(loop);
    }
    
    requestAnimationFrame(loop);
}

let randomMap = maps[Math.floor(Math.random() * maps.length)];
console.log(randomMap);
playGame(randomMap.garbCols, randomMap.queue);
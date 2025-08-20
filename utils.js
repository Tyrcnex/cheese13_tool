class PieceLocation {
    constructor(piece, offset, rotation) {
        this.piece = piece;
        this._minos = PIECES[piece].minos;
        this.offset = offset;
        this.rotation = rotation;
    }

    minos() {
        let m = this._minos.map(e =>
            this.rotation == 0 ? [e[0], e[1]] :
                this.rotation == 1 ? [e[1], -e[0]] :
                    this.rotation == 2 ? [-e[0], -e[1]] :
                        [-e[1], e[0]]
        );
        return m.map(e => [e[0] + this.offset[0], e[1] + this.offset[1]]);
    }

    clone() {
        return new PieceLocation(this.piece, [...this.offset], this.rotation);
    }
}

class Board {
    constructor(currentPiece, dta = [...Array(40)].map(_ => Array(10).fill(0))) {
        this.dta = dta;
        this.currentPiece = currentPiece;
    }

    // gets idxs of all lines that *weren't* cleared
    lineClearMask() {
        let mask = [];
        for (let y = 0; y < 40; y++) {
            if (!this.dta[y].every(c => c != 0)) {
                mask.push(y);
            }
        }
        return mask;
    }

    clearLines() {
        let mask = this.lineClearMask();
        let newDta = [];
        for (const m of mask) {
            newDta.push(this.dta[m]);
        }
        for (let m = mask.length; m < 40; m++) {
            newDta.push(Array(10).fill(0));
        }
        this.dta = newDta;
    }

    pushGarbage(idx) {
        let garbageRow = Array(10).fill(0);
        for (let x = 0; x < 10; x++)
            garbageRow[x] = -(idx != x);
        this.dta.unshift(garbageRow);
        this.dta.pop();
    }

    placeMinos() {
        for (const mino of this.currentPiece.minos()) {
            this.dta[mino[1]][mino[0]] = ORDER.indexOf(this.currentPiece.piece) + 1;
        }
        this.clearLines();
    }

    obstructed(minos) {
        for (const mino of minos)
            if (mino[0] < 0 || mino[0] > 9 || mino[1] < 0 || this.dta[mino[1]][mino[0]] != 0)
                return true;

        return false;
    }

    dist(stepFunc = x => x[1]--) {
        const testPiece = this.currentPiece.clone();
        let count = 0;
        while (!this.obstructed(testPiece.minos())) {
            count++;
            stepFunc(testPiece.offset);
        }
        return Math.max(0, count - 1);
    }

    moveX(x) {
        let stepFunc = e => e[0] += (x > 0 ? 1 : -1);
        let d = this.dist(stepFunc);
        for (let i = 0; i < Math.min(d, Math.abs(x)); i++)
            stepFunc(this.currentPiece.offset);
    }

    moveY(y) {
        let stepFunc = e => e[1] += (y > 0 ? 1 : -1);
        let d = this.dist(stepFunc);
        for (let i = 0; i < Math.min(d, Math.abs(y)); i++)
            stepFunc(this.currentPiece.offset);
    }

    rotate(o) {
        if (this.currentPiece.piece == "O") return;

        let from = this.currentPiece.rotation;
        let to = (((from + o) % 4) + 4) % 4;
        let ttt = 0;
        for (const kicks of KICKS[this.currentPiece.piece == "I" ? "iKicks" : "kicks"]["" + from + to]) {
            let testPiece = this.currentPiece.clone();
            testPiece.offset[0] += kicks[0];
            testPiece.offset[1] += kicks[1];
            testPiece.rotation = to;
            if (!this.obstructed(testPiece.minos())) {
                this.currentPiece.offset[0] += kicks[0];
                this.currentPiece.offset[1] += kicks[1];
                this.currentPiece.rotation = to;
                return;
            }
        }
    }

    draw(ctx) {
        for (let x = 0; x < 10; x++) {
            for (let y = 0; y < 20; y++) {
                let boardMino = this.dta[y][x];
                ctx.fillStyle = boardMino == -1 ? "#a3a3a3" : boardMino == 0 ? "#000" : PIECES[ORDER[boardMino - 1]].color;
                ctx.fillRect(150 + 30 * x, 30 * (20 - y), 30, 30);
            }
        }

        if (this.currentPiece) {
            let shadowPiece = this.currentPiece.clone();
            shadowPiece.offset[1] -= this.dist();

            ctx.fillStyle = "#444444ff";
            for (const mino of shadowPiece.minos()) {
                ctx.fillRect(150 + 30 * mino[0], 30 * (20 - mino[1]), 30, 30);
            }
            ctx.fillStyle = PIECES[this.currentPiece.piece].color;
            for (const mino of this.currentPiece.minos()) {
                ctx.fillRect(150 + 30 * mino[0], 30 * (20 - mino[1]), 30, 30);
            }
        }
    }
}

function appendQueue(q) {
    let pieces = ORDER.split("");
    for (let i = 6; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [pieces[i], pieces[j]] = [pieces[j], pieces[i]];
    }
    q.push(...pieces);
}

if (!localStorage.KEYS) {
    localStorage.setItem("KEYS", JSON.stringify(DEFAULT_KEYS));
} else {
    const newKeys = JSON.parse(localStorage.KEYS);
    for (const key in DEFAULT_KEYS) {
        if (!newKeys[key]) {
            newKeys[key] = DEFAULT_KEYS[key];
        }
    }
    const allKeys = Object.values(newKeys).flat();
    if ((new Set(allKeys)).size < allKeys.length) {
        alert("config clash detected! please modify localStorage.KEYS in dev console");
    }
    localStorage.setItem("KEYS", JSON.stringify(newKeys));
}

if (!localStorage.HANDLING) {
    localStorage.setItem("HANDLING", JSON.stringify(DEFAULT_HANDLING));
}

const vKey = (type, key) => JSON.parse(localStorage.KEYS)[type]?.some(x => x.toLowerCase() == key.toLowerCase());
const handl = v => JSON.parse(localStorage.HANDLING)[v];
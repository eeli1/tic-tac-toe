function setup() {
    createCanvas(310, 310);
    this.gui = new GUI(
        10,
        100,
        color(255, 0, 0),
        color(0, 0, 255),
        color(0, 0, 0)
    );

    this.gui.drawGrid();
    this.player = "X";
}

function draw() {}

function mouseClicked() {
    let x = int(mouseX / 100);
    let y = int(mouseY / 100);

    if (this.player == "X") {
        this.gui.drawX(x, y);
        this.player = "O";
    } else {
        this.gui.drawO(x, y);
        this.player = "X";
    }
}

class GUI {
    constructor(offset, boxLen, oColor, xColor, gridColor) {
        this.offset = offset;
        this.boxLen = boxLen;

        this.oColor = oColor;
        this.xColor = xColor;
        this.gridColor = gridColor;
    }

    drawGrid() {
        stroke(this.gridColor);
        line(
            this.offset,
            this.boxLen + this.offset,
            3 * this.boxLen + this.offset,
            this.boxLen + this.offset
        );
        line(
            this.offset,
            2 * this.boxLen + this.offset,
            3 * this.boxLen + this.offset,
            2 * this.boxLen + this.offset
        );

        line(
            this.boxLen + this.offset,
            this.offset,
            this.boxLen + this.offset,
            3 * this.boxLen + this.offset
        );
        line(
            2 * this.boxLen + this.offset,
            this.offset,
            2 * this.boxLen + this.offset,
            3 * this.boxLen + this.offset
        );
    }

    drawO(x, y) {
        x *= this.boxLen;
        y *= this.boxLen;

        x += this.offset;
        y += this.offset;

        stroke(this.oColor);
        ellipse(
            x + this.boxLen / 2,
            y + this.boxLen / 2,
            this.boxLen - 3,
            this.boxLen - 3
        );
    }

    drawX(x, y) {
        x *= this.boxLen;
        y *= this.boxLen;

        x += this.offset;
        y += this.offset;

        stroke(this.xColor);
        line(x, y, x + this.boxLen, y + this.boxLen);
        line(x, y + this.boxLen, x + this.boxLen, y);
    }
}
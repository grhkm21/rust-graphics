import { draw, start, Universe } from "./pkg";

let ctx = start();
draw(ctx, 0);

let cnt = 1;
setInterval(() => {
    draw(ctx, cnt);
    cnt += 1;
}, 50);

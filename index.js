import { draw, start, House } from "./pkg";

let ctx = start();
draw(ctx, 0);

let house = House.new(8.0);

let cnt = 1;
setInterval(() => {
    house.tick();
    let pos_x = house.get_x();
    let pos_y = house.get_y();
    console.log("pos = %f, %f", pos_x, pos_y);
    draw(ctx, pos_x, pos_y);
    cnt += 1;
}, 50);

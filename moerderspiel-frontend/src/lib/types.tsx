export type Order = {
    id: number,
    circle: number,
    target_id: number,
    target_name: string,
    target_variant: number,
};

export type PlayerState = {
    id: number,
    name: string,
    variant: number,
    num_kills: number,
    orders: Order[],
};

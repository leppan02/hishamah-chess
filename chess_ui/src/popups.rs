use super::*;
use orbtk::prelude::*;

pub fn popup_win(id: Entity, ctx: &mut BuildContext, text: String) -> Entity {
    Popup::new()
        .target(id)
        .open(true)
        .child(
            Container::new()
                .background(walkable_tiles.1)
                .h_align("center")
                .v_align("center")
                .padding(20)
                .child(
                    Grid::new()
                        .rows(Rows::create().push("auto").push(50).push("*"))
                        .child(
                            TextBlock::new()
                                .text(text)
                                .font_size(30)
                                .attach(Grid::row(0))
                                .h_align("center")
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .attach(Grid::row(2))
                                .text("Play agane")
                                .on_click(move |state, _| {
                                    let cs: &mut ChessState = state.get_mut(id);
                                    cs.action(Action::Restart);

                                    true
                                })
                                .h_align("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}

pub fn popup_promote(id: Entity, ctx: &mut BuildContext) -> Entity {
    Popup::new()
        .target(id)
        .open(true)
        .child(
            Container::new()
                .h_align("center")
                .v_align("center")
                .child(
                    Stack::new()
                        .h_align("center")
                        .spacing(8)
                        .child(
                            TextBlock::new()
                                .text("Promote into")
                                .font_size(48)
                                .build(ctx),
                        )
                        .child(promote_tile(id, ctx, "Queen".to_owned(), PieceType::Queen))
                        .child(promote_tile(
                            id,
                            ctx,
                            "Bishop".to_owned(),
                            PieceType::Bishop,
                        ))
                        .child(promote_tile(id, ctx, "Rook".to_owned(), PieceType::Rook))
                        .child(promote_tile(
                            id,
                            ctx,
                            "Knight".to_owned(),
                            PieceType::Knight,
                        ))
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}

fn promote_tile(id: Entity, ctx: &mut BuildContext, text: String, kind: PieceType) -> Entity {
    Button::new()
        .text(text)
        .font_size(30)
        .on_click(move |state, _| {
            let cs: &mut ChessState = state.get_mut(id);
            cs.action(Action::PromoteTile(kind));

            true
        })
        .build(ctx)
}

pub fn popup_start(id: Entity, ctx: &mut BuildContext, ip: String) {}

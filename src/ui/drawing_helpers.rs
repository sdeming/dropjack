use crate::game::Game;
use crate::models::Card;
use crate::ui::background_renderer::BackgroundRenderer;
use crate::ui::card_renderer::CardRenderer;
use crate::ui::instruction_renderer::InstructionRenderer;
use crate::ui::menu_renderer::MenuRenderer;
use crate::ui::text_renderer::TextRenderer;

use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::{Font, Texture2D};

pub struct DrawingHelpers;

impl DrawingHelpers {
    // Re-export card rendering functions with improved API
    pub fn draw_card_inline(
        d: &mut RaylibDrawHandle,
        atlas: &Texture2D,
        card: Card,
        card_x: i32,
        card_y: i32,
        size: i32,
    ) {
        CardRenderer::draw_card_inline(d, atlas, card, card_x, card_y, size);
    }

    // Re-export background rendering functions
    pub fn draw_gradient_background(d: &mut RaylibDrawHandle) {
        BackgroundRenderer::draw_gradient_background(d);
    }

    pub fn draw_game_board_background(
        d: &mut RaylibDrawHandle,
        board_width: i32,
        board_height: i32,
        cell_size: i32,
    ) {
        BackgroundRenderer::draw_game_board_background(d, board_width, board_height, cell_size);
    }

    // Re-export text rendering functions
    pub fn draw_title_with_shadow(d: &mut RaylibDrawHandle, title_font: &Font) {
        TextRenderer::draw_title_with_shadow(d, title_font);
    }

    pub fn draw_subtitle(d: &mut RaylibDrawHandle, font: &Font) {
        TextRenderer::draw_subtitle(d, font);
    }

    // Re-export menu rendering functions
    pub fn draw_main_panel(d: &mut RaylibDrawHandle) {
        MenuRenderer::draw_main_panel(d);
    }

    pub fn draw_difficulty_selector(
        d: &mut RaylibDrawHandle,
        title_font: &Font,
        font: &Font,
        game: &Game,
        has_controller: bool,
    ) {
        MenuRenderer::draw_difficulty_selector(d, title_font, font, game, has_controller);
    }

    pub fn draw_high_scores_panel(
        d: &mut RaylibDrawHandle,
        title_font: &Font,
        font: &Font,
        game: &Game,
    ) {
        MenuRenderer::draw_high_scores_panel(d, title_font, font, game);
    }

    pub fn draw_start_button(d: &mut RaylibDrawHandle, title_font: &Font, has_controller: bool) {
        MenuRenderer::draw_start_button(d, title_font, has_controller);
    }

    // Re-export instruction rendering functions
    pub fn draw_controls(
        d: &mut RaylibDrawHandle,
        title_font: &Font,
        font: &Font,
        info_panel_x: i32,
        board_offset_y: i32,
        has_controller: bool,
    ) {
        InstructionRenderer::draw_controls(
            d,
            title_font,
            font,
            info_panel_x,
            board_offset_y,
            has_controller,
        );
    }

    pub fn draw_game_over_instructions(
        d: &mut RaylibDrawHandle,
        font: &Font,
        has_controller: bool,
    ) {
        InstructionRenderer::draw_game_over_instructions(d, font, has_controller);
    }

    pub fn draw_quit_confirmation(d: &mut RaylibDrawHandle, font: &Font, has_controller: bool) {
        InstructionRenderer::draw_quit_confirmation(d, font, has_controller);
    }

    pub fn draw_pause_instructions(d: &mut RaylibDrawHandle, font: &Font, has_controller: bool) {
        InstructionRenderer::draw_pause_instructions(d, font, has_controller);
    }
}

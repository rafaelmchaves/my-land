pub mod game {
    use bevy::prelude::*;

    use crate::core::generate_next_turn;
    
    use crate::repository::infrastructure::{self, Infrastructures};

    use crate::ui::{despawn_screen, GameState, TEXT_COLOR};

    const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    fn game_button(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color) in &mut interaction_query {
            *color = match *interaction {
                Interaction::Pressed | Interaction::None => PRESSED_BUTTON.into(),
                Interaction::Hovered => HOVERED_PRESSED_BUTTON.into()
            }
        }
    }

    #[derive(Component)]
    enum ButtonEventsAction {
        Advance,
        BackMenu,
        Infrastructure,
    }

    #[derive(States, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
    enum ClickedAction {
        #[default]
        NothingClicked,
        Infrastructure
    }

    fn game_button_events(
        interaction_query: Query<
            (&Interaction, &ButtonEventsAction),
            (Changed<Interaction>, With<Button>),
        >,

        mut game_state: ResMut<NextState<GameState>>,
        mut button_clicked:  ResMut<NextState<ClickedAction>>
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    ButtonEventsAction::Advance => {generate_next_turn()},
                    ButtonEventsAction::BackMenu => {
                        game_state.set(GameState::Menu);
                    }
                    ButtonEventsAction::Infrastructure => {button_clicked.set(ClickedAction::Infrastructure)}
                }
            }
        }
    }

    // This plugin will contain the game.
    pub struct GamePlugin;
    use crate::core;

    impl Plugin for GamePlugin {

        fn build(&self, app: &mut App) {

            let infras = Infrastructures { list: (infrastructure::get_all_infrastructures()).unwrap() };
            app.add_systems(OnEnter(GameState::Game), game_setup)
                .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
                .add_systems(
                Update, 
                (game_button_events, game_button).run_if(in_state(GameState::Game)))
                .insert_resource(infras)
                .add_state::<ClickedAction>()
                .add_systems(OnEnter(ClickedAction::Infrastructure), core::add_game_infra );
        }
    }

    
    // Tag component used to tag entities added on the game screen
    #[derive(Component)]
    struct OnGameScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);

    fn game_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {

        let button_style = Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_game_actions_style = Style {
            width: Val::Px(52.0),
            height: Val::Px(35.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_icon_style = Style {
            width: Val::Px(30.0),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: Val::Px(10.0),
            ..default()
        };
        let button_text_style = TextStyle {
            font_size: 40.0,
            color: TEXT_COLOR,
            ..default()
        };
        const ADVANCE_BUTTON: Color = Color::rgb(0.22, 0.15, 0.15);
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        // center children
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnGameScreen,
            ))
            .with_children(|parent| {

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            // This will display its children in a column, from top to bottom
                            flex_direction: FlexDirection::Column,
                            // `align_items` will align children on the cross axis. Here the main axis is
                            // vertical (column), so the cross axis is horizontal. This will center the
                            // children
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    }).with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {

                        //Create the horizontal bar with buttom actions(policies, budget and build etc) in the top of the screen
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::BLACK.into(),
                            ..default()
                        }).with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
                            parent.spawn((ButtonBundle {
                                    style: button_game_actions_style.clone(),
                                    background_color: ADVANCE_BUTTON.into(),
                                    ..default()
                                },
                                ButtonEventsAction::Infrastructure,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("infra.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                            });
                            parent.spawn((ButtonBundle {
                                style: button_game_actions_style.clone(),
                                background_color: ADVANCE_BUTTON.into(),
                                ..default()
                            },
                            ButtonEventsAction::Advance,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("budget.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                            });
                            parent.spawn((ButtonBundle {
                                style: button_game_actions_style,
                                background_color: ADVANCE_BUTTON.into(),
                                ..default()
                            },
                                ButtonEventsAction::Advance,
                            )).with_children(|parent| {
                                let icon = asset_server.load("policies.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                            });
                        });

                        //Create the game area
                        parent.spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::RED.into(),
                            ..default()
                        })
                    
                        .with_children(|parent| {
                            // Display the game name
                            parent.spawn(
                                TextBundle::from_section(
                                    "Game Area",
                                    TextStyle {
                                        font_size: 80.0,
                                        color: TEXT_COLOR,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(350.0)),
                                    ..default()
                                }),
                            );
                        });
                    });
                // Create the action menu in the bottom with the Advance and back to menu buttons
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            // This will display its children in a column, from top to bottom
                            flex_direction: FlexDirection::Column,
                            // `align_items` will align children on the cross axis. Here the main axis is
                            // vertical (column), so the cross axis is horizontal. This will center the
                            // children
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Display a advanced button in the right bottom
                        parent.spawn(NodeBundle {
                            style: Style {
                                // This will display its children in a column, from top to bottom
                                flex_direction: FlexDirection::Column,
                                // `align_items` will align children on the cross axis. Here the main axis is
                                // vertical (column), so the cross axis is horizontal. This will center the
                                // children
                                align_items: AlignItems::End,
                                ..default()
                            },
                            background_color: Color::BLACK.into(),
                            ..default()
                        }).with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
                            parent.spawn((ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: ADVANCE_BUTTON.into(),
                                    ..default()
                                },
                                ButtonEventsAction::Advance,
                              ))
                            .with_children(|parent| {
                                let icon = asset_server.load("right.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Advance",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((ButtonBundle {
                                  style: button_style.clone(),
                                  background_color: ADVANCE_BUTTON.into(),
                                  ..default()
                              }, ButtonEventsAction::BackMenu
                            ))
                          .with_children(|parent| {
                              parent.spawn(TextBundle::from_section(
                                  "Back to menu",
                                  button_text_style.clone(),
                              ));
                          });
                        });
                        
                    });
            });
  
    }

}

use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

#[derive(Component, Default, Clone, PartialEq, Eq)]
pub struct MyWidgetProps {}

impl Widget for MyWidgetProps {}

#[derive(Bundle)]
pub struct MyWidgetBundle {
    props: MyWidgetProps,
    styles: KStyle,
    computed_styles: ComputedStyles,
    widget_name: WidgetName,
}

impl Default for MyWidgetBundle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            styles: Default::default(),
            computed_styles: Default::default(),
            widget_name: MyWidgetProps::default().get_name(),
        }
    }
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_resource<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((widget_context, entity, previous_entity)): In<(KayakWidgetContext, Entity, Entity)>,
    keys: Res<Input<KeyCode>>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    widget_param.has_changed(&widget_context, entity, previous_entity)
        || keys.just_pressed(KeyCode::Tab)
        || keys.just_released(KeyCode::Tab)
    // true
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kttf"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_data::<MyWidgetProps, EmptyState>();
    widget_context.add_widget_system(
        MyWidgetProps::default().get_name(),
        widget_update_with_resource::<MyWidgetProps, EmptyState>,
        my_widget_render,
    );
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <MyWidgetBundle />
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}

fn my_widget_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
) -> bool {
    if keys.pressed(KeyCode::Tab) {
        info!("rendering text widget");
        let parent_id = Some(entity);
        rsx! {
            <TextWidgetBundle
                text={TextProps {
                    content: "Hello World".into(),
                    size: 20.0,
                    ..Default::default()
                }}
            />
        };
    } else {
        info!("not rendering anything");
    }

    true
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(startup)
        .run()
}

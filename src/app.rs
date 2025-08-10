use poc_fnb::{Ingredient, IngredientType, Plate, PlateComponent, QuantityUnit, Supplier};
use std::cell::RefCell;
use std::rc::Rc;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    // F&B Management data (skipped from serialization for now due to complexity)
    #[serde(skip)]
    ingredients: Vec<Rc<RefCell<Ingredient>>>,

    #[serde(skip)]
    plates: Vec<Rc<RefCell<Plate>>>,

    // UI state
    #[serde(skip)]
    selected_ingredient_idx: Option<usize>,

    #[serde(skip)]
    selected_plate_idx: Option<usize>,

    #[serde(skip)]
    new_ingredient_name: String,

    #[serde(skip)]
    new_ingredient_price: f64,

    #[serde(skip)]
    new_ingredient_quantity: f64,

    #[serde(skip)]
    new_ingredient_unit: QuantityUnit,

    #[serde(skip)]
    new_ingredient_type: IngredientType,

    // New recipe creation state
    #[serde(skip)]
    new_recipe_name: String,

    #[serde(skip)]
    new_recipe_selling_price: f64,

    #[serde(skip)]
    new_recipe_prep_time: f64,

    #[serde(skip)]
    new_recipe_batch_quantity: f64,

    #[serde(skip)]
    new_recipe_batch_unit: QuantityUnit,

    #[serde(skip)]
    editing_recipe_idx: Option<usize>,

    #[serde(skip)]
    show_add_recipe_form: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let mut app = Self {
            // Example stuff:
            label: "Gestion des Coûts F&B".to_owned(),
            value: 2.7,
            ingredients: Vec::new(),
            plates: Vec::new(),
            selected_ingredient_idx: None,
            selected_plate_idx: None,
            new_ingredient_name: String::new(),
            new_ingredient_price: 0.0,
            new_ingredient_quantity: 100.0,
            new_ingredient_unit: QuantityUnit::Gram,
            new_ingredient_type: IngredientType::Grocery,
            new_recipe_name: String::new(),
            new_recipe_selling_price: 0.0,
            new_recipe_prep_time: 0.5,
            new_recipe_batch_quantity: 1.0,
            new_recipe_batch_unit: QuantityUnit::Unit,
            editing_recipe_idx: None,
            show_add_recipe_form: false,
        };

        // Initialize with sample data
        app.initialize_sample_data();
        app
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut app: TemplateApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            // Reinitialize sample data since it's not serialized
            app.initialize_sample_data();
            app
        } else {
            Default::default()
        }
    }

    fn initialize_sample_data(&mut self) {
        // Create suppliers
        let supplier1 = Supplier {
            name: "FermesFraîches".to_string(),
            contact: Some("contact@fermesfraîches.fr".to_string()),
        };

        let supplier2 = Supplier {
            name: "LaiterieBest".to_string(),
            contact: Some("info@laiteriebest.fr".to_string()),
        };

        // Create ingredients
        let tomato = Rc::new(RefCell::new(Ingredient {
            name: "Tomate".to_string(),
            cost_price: 0.5,
            reference_quantity: 100.0,
            unit: QuantityUnit::Gram,
            ingredient_type: IngredientType::VegetablesFruits,
            supplier: supplier1.clone(),
        }));

        let cheese = Rc::new(RefCell::new(Ingredient {
            name: "Fromage".to_string(),
            cost_price: 1.0,
            reference_quantity: 100.0,
            unit: QuantityUnit::Gram,
            ingredient_type: IngredientType::DairyEggsCheeseSauces,
            supplier: supplier2,
        }));

        let flour = Rc::new(RefCell::new(Ingredient {
            name: "Farine".to_string(),
            cost_price: 2.0,
            reference_quantity: 1.0,
            unit: QuantityUnit::Kilogram,
            ingredient_type: IngredientType::Grocery,
            supplier: supplier1,
        }));

        self.ingredients = vec![tomato.clone(), cheese.clone(), flour.clone()];

        // Create tomato sauce plate
        let tomato_sauce = Rc::new(RefCell::new(Plate {
            name: "Sauce Tomate".to_string(),
            components: vec![PlateComponent::Ingredient {
                ingredient: tomato.clone(),
                quantity: 1500.0,
                unit: QuantityUnit::Gram,
            }],
            selling_price: 5.0,
            batch_preparation_time_hours: 0.25,
            batch_quantity: 1500.0,
            batch_unit: QuantityUnit::Gram,
        }));

        // Create pizza plate
        let pizza = Rc::new(RefCell::new(Plate {
            name: "Pizza Margherita".to_string(),
            components: vec![
                PlateComponent::Plate {
                    plate: tomato_sauce.clone(),
                    quantity: 150.0,
                },
                PlateComponent::Ingredient {
                    ingredient: cheese.clone(),
                    quantity: 50.0,
                    unit: QuantityUnit::Gram,
                },
                PlateComponent::Ingredient {
                    ingredient: flour.clone(),
                    quantity: 200.0,
                    unit: QuantityUnit::Gram,
                },
            ],
            selling_price: 12.0,
            batch_preparation_time_hours: 0.5,
            batch_quantity: 1.0,
            batch_unit: QuantityUnit::Unit,
        }));

        self.plates = vec![tomato_sauce, pizza];
    }

    fn get_unit_variants() -> Vec<QuantityUnit> {
        vec![
            QuantityUnit::Gram,
            QuantityUnit::Kilogram,
            QuantityUnit::Milligram,
            QuantityUnit::Liter,
            QuantityUnit::Milliliter,
            QuantityUnit::Centiliter,
            QuantityUnit::Deciliter,
            QuantityUnit::Unit,
            QuantityUnit::Piece,
            QuantityUnit::Slice,
            QuantityUnit::Teaspoon,
            QuantityUnit::Tablespoon,
            QuantityUnit::Cup,
        ]
    }

    fn create_new_recipe(&mut self) {
        if !self.new_recipe_name.is_empty() {
            let new_recipe = Rc::new(RefCell::new(Plate {
                name: self.new_recipe_name.clone(),
                components: Vec::new(),
                selling_price: self.new_recipe_selling_price,
                batch_preparation_time_hours: self.new_recipe_prep_time,
                batch_quantity: self.new_recipe_batch_quantity,
                batch_unit: self.new_recipe_batch_unit.clone(),
            }));

            self.plates.push(new_recipe);

            // Reset form
            self.new_recipe_name.clear();
            self.new_recipe_selling_price = 0.0;
            self.new_recipe_prep_time = 0.5;
            self.new_recipe_batch_quantity = 1.0;
            self.new_recipe_batch_unit = QuantityUnit::Unit;
            self.show_add_recipe_form = false;
        }
    }

    fn ingredients_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Ingrédients");

        // Add new ingredient form
        ui.collapsing("Ajouter un Nouvel Ingrédient", |ui| {
            ui.horizontal(|ui| {
                ui.label("Nom :");
                ui.text_edit_singleline(&mut self.new_ingredient_name);
            });

            ui.horizontal(|ui| {
                ui.label("Prix :");
                ui.add(
                    egui::DragValue::new(&mut self.new_ingredient_price)
                        .speed(0.01)
                        .prefix("€"),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Quantité Réf. :");
                ui.add(egui::DragValue::new(&mut self.new_ingredient_quantity).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Unité :");
                egui::ComboBox::from_id_salt("ingredient_unit_combo")
                    .selected_text(match self.new_ingredient_unit {
                        QuantityUnit::Gram => "Gramme",
                        QuantityUnit::Kilogram => "Kilogramme",
                        QuantityUnit::Milligram => "Milligramme",
                        QuantityUnit::Liter => "Litre",
                        QuantityUnit::Milliliter => "Millilitre",
                        QuantityUnit::Centiliter => "Centilitre",
                        QuantityUnit::Deciliter => "Décilitre",
                        QuantityUnit::Unit => "Unité",
                        QuantityUnit::Piece => "Pièce",
                        QuantityUnit::Slice => "Tranche",
                        QuantityUnit::Teaspoon => "Cuillère à café",
                        QuantityUnit::Tablespoon => "Cuillère à soupe",
                        QuantityUnit::Cup => "Tasse",
                        _ => "Autre",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Gram,
                            "Gramme",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Kilogram,
                            "Kilogramme",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Milligram,
                            "Milligramme",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Liter,
                            "Litre",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Milliliter,
                            "Millilitre",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Centiliter,
                            "Centilitre",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Deciliter,
                            "Décilitre",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Unit,
                            "Unité",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Piece,
                            "Pièce",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Slice,
                            "Tranche",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Teaspoon,
                            "Cuillère à café",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Tablespoon,
                            "Cuillère à soupe",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_unit,
                            QuantityUnit::Cup,
                            "Tasse",
                        );
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Type :");
                egui::ComboBox::from_id_salt("ingredient_type_combo")
                    .selected_text(match self.new_ingredient_type {
                        IngredientType::Grocery => "Épicerie",
                        IngredientType::DairyEggsCheeseSauces => "Lait, Œufs, Fromages, Sauces",
                        IngredientType::VegetablesFruits => "Légumes & Fruits",
                        IngredientType::Packaging => "Packaging",
                        IngredientType::MeatProteins => "Viandes / Protéines",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.new_ingredient_type,
                            IngredientType::Grocery,
                            "Épicerie",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_type,
                            IngredientType::DairyEggsCheeseSauces,
                            "Lait, Œufs, Fromages, Sauces",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_type,
                            IngredientType::VegetablesFruits,
                            "Légumes & Fruits",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_type,
                            IngredientType::Packaging,
                            "Packaging",
                        );
                        ui.selectable_value(
                            &mut self.new_ingredient_type,
                            IngredientType::MeatProteins,
                            "Viandes / Protéines",
                        );
                    });
            });

            if ui.button("Ajouter Ingrédient").clicked() && !self.new_ingredient_name.is_empty() {
                let new_ingredient = Rc::new(RefCell::new(Ingredient {
                    name: self.new_ingredient_name.clone(),
                    cost_price: self.new_ingredient_price,
                    reference_quantity: self.new_ingredient_quantity,
                    unit: self.new_ingredient_unit.clone(),
                    ingredient_type: self.new_ingredient_type.clone(),
                    supplier: Supplier {
                        name: "Par Défaut".to_string(),
                        contact: None,
                    },
                }));

                self.ingredients.push(new_ingredient);
                self.new_ingredient_name.clear();
                self.new_ingredient_price = 0.0;
                self.new_ingredient_quantity = 100.0;
                self.new_ingredient_unit = QuantityUnit::Gram;
                self.new_ingredient_type = IngredientType::Grocery;
            }
        });

        ui.separator();

        // Ingredients list
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, ingredient_rc) in self.ingredients.iter().enumerate() {
                let mut ingredient = ingredient_rc.borrow_mut();

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading(&ingredient.name);
                        if ui.small_button("🗑").clicked() {
                            // Mark for deletion (we'll handle this outside the borrow)
                            self.selected_ingredient_idx = Some(idx);
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Prix de Revient :");
                        let response = ui.add(
                            egui::DragValue::new(&mut ingredient.cost_price)
                                .speed(0.01)
                                .prefix("€"),
                        );

                        if response.changed() {
                            // Price changed, UI will automatically reflect in plates
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Quantité de Référence :");
                        ui.add(egui::DragValue::new(&mut ingredient.reference_quantity).speed(1.0));
                        ui.label(format!("{:?}", ingredient.unit));
                    });

                    ui.label(format!("Fournisseur : {}", ingredient.supplier.name));
                    ui.label(format!("Type : {:?}", ingredient.ingredient_type));
                });
            }
        });

        // Handle deletion outside of borrow
        if let Some(idx) = self.selected_ingredient_idx.take() {
            if idx < self.ingredients.len() {
                self.ingredients.remove(idx);
            }
        }
    }

    fn plates_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Plats (Recettes)");

        // Add new recipe button
        ui.horizontal(|ui| {
            if ui.button("➕ Ajouter une Nouvelle Recette").clicked() {
                self.show_add_recipe_form = true;
            }
        });

        // Add new recipe form
        if self.show_add_recipe_form {
            ui.group(|ui| {
                ui.heading("Créer une Nouvelle Recette");

                ui.horizontal(|ui| {
                    ui.label("Nom :");
                    ui.text_edit_singleline(&mut self.new_recipe_name);
                });

                ui.horizontal(|ui| {
                    ui.label("Prix de Vente :");
                    ui.add(
                        egui::DragValue::new(&mut self.new_recipe_selling_price)
                            .speed(0.1)
                            .prefix("€"),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Temps de Préparation (h) :");
                    ui.add(
                        egui::DragValue::new(&mut self.new_recipe_prep_time)
                            .speed(0.1)
                            .range(0.0..=24.0),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Quantité par Lot :");
                    ui.add(
                        egui::DragValue::new(&mut self.new_recipe_batch_quantity)
                            .speed(0.1)
                            .range(0.1..=1000.0),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Unité du Lot :");
                    egui::ComboBox::from_id_salt("new_recipe_unit")
                        .selected_text(format!("{:?}", self.new_recipe_batch_unit))
                        .show_ui(ui, |ui| {
                            for unit in Self::get_unit_variants() {
                                ui.selectable_value(
                                    &mut self.new_recipe_batch_unit,
                                    unit.clone(),
                                    format!("{:?}", unit),
                                );
                            }
                        });
                });

                ui.horizontal(|ui| {
                    if ui.button("Créer la Recette").clicked() {
                        self.create_new_recipe();
                    }
                    if ui.button("Annuler").clicked() {
                        self.show_add_recipe_form = false;
                    }
                });
            });
        }

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, plate_rc) in self.plates.iter().enumerate() {
                let mut plate = plate_rc.borrow_mut();

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading(&plate.name);
                        if ui.small_button("🗑").clicked() {
                            self.selected_plate_idx = Some(idx);
                        }
                    });

                    // Show cost breakdown
                    ui.horizontal(|ui| {
                        ui.label("Coût Ingrédients :");
                        ui.label(format!("€{:.2}", plate.ingredient_cost()));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Coût Main-d'œuvre :");
                        ui.label(format!("€{:.2}", plate.labor_cost()));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Coût Total :");
                        ui.colored_label(
                            egui::Color32::from_rgb(255, 100, 100),
                            format!("€{:.2}", plate.total_cost_price()),
                        );
                    });

                    // Editable selling price
                    ui.horizontal(|ui| {
                        ui.label("Prix de Vente :");
                        ui.add(
                            egui::DragValue::new(&mut plate.selling_price)
                                .speed(0.1)
                                .prefix("€"),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("Marge Brute :");
                        ui.colored_label(
                            egui::Color32::from_rgb(100, 255, 100),
                            format!("€{:.2}", plate.gross_margin()),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("Taux de Marge :");
                        ui.colored_label(
                            egui::Color32::from_rgb(100, 255, 100),
                            format!("{:.1}%", plate.margin_rate() * 100.0),
                        );
                    });

                    // Show and edit components
                    egui::CollapsingHeader::new("Composants")
                        .id_salt(format!("components_plate_{}", plate.name))
                        .show(ui, |ui| {
                            // Add ingredient to recipe
                            ui.horizontal(|ui| {
                                ui.label("Ajouter un ingrédient :");
                                egui::ComboBox::from_id_salt(format!("add_ingredient_{}", idx))
                                    .selected_text("Sélectionner un ingrédient...")
                                    .show_ui(ui, |ui| {
                                        for ingredient_rc in &self.ingredients {
                                            let ingredient = ingredient_rc.borrow();
                                            if ui
                                                .selectable_label(false, &ingredient.name)
                                                .clicked()
                                            {
                                                // Add ingredient to recipe with default quantity
                                                plate.components.push(PlateComponent::Ingredient {
                                                    ingredient: ingredient_rc.clone(),
                                                    quantity: 100.0,
                                                    unit: QuantityUnit::Gram,
                                                });
                                            }
                                        }
                                    });
                            });

                            // Display and edit existing components
                            let mut components_to_remove = Vec::new();
                            for (comp_idx, component) in plate.components.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    match component {
                                        PlateComponent::Ingredient {
                                            ingredient,
                                            quantity,
                                            unit: _,
                                        } => {
                                            let ing = ingredient.borrow();
                                            ui.label(&ing.name);
                                            ui.add(
                                                egui::DragValue::new(quantity)
                                                    .speed(1.0)
                                                    .suffix("g")
                                                    .range(0.1..=10000.0),
                                            );

                                            // Convert ingredient unit to grams for display
                                            let cost_per_gram = if ing.unit == QuantityUnit::Gram {
                                                ing.cost_price / ing.reference_quantity
                                            } else if ing.unit == QuantityUnit::Kilogram {
                                                // Convert kg to g: 1 kg = 1000 g
                                                ing.cost_price / (ing.reference_quantity * 1000.0)
                                            } else {
                                                // For other units, use conversion if available
                                                if let Some(conversion_factor) = ing
                                                    .unit
                                                    .conversion_factor_to(&QuantityUnit::Gram)
                                                {
                                                    (ing.cost_price / ing.reference_quantity)
                                                        * conversion_factor
                                                } else {
                                                    ing.cost_price / ing.reference_quantity
                                                }
                                            };

                                            ui.label(format!("@ €{:.4}/g", cost_per_gram));
                                        }
                                        PlateComponent::Plate {
                                            plate: sub_plate,
                                            quantity,
                                        } => {
                                            let sub = sub_plate.borrow();
                                            ui.label(&sub.name);
                                            ui.add(
                                                egui::DragValue::new(quantity)
                                                    .speed(1.0)
                                                    .suffix("g")
                                                    .range(0.1..=10000.0),
                                            );
                                            ui.label(format!(
                                                "@ €{:.4}/g",
                                                sub.batch_cost_per_unit()
                                            ));
                                        }
                                    }
                                    if ui.small_button("🗑").clicked() {
                                        components_to_remove.push(comp_idx);
                                    }
                                });
                            }

                            // Remove components marked for deletion
                            for &idx_to_remove in components_to_remove.iter().rev() {
                                plate.components.remove(idx_to_remove);
                            }
                        });
                });
            }
        });

        // Handle plate deletion outside of borrow
        if let Some(idx) = self.selected_plate_idx.take() {
            if idx < self.plates.len() {
                self.plates.remove(idx);
            }
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("Fichier", |ui| {
                        if ui.button("Quitter").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("ingredients_panel")
            .min_width(400.0)
            .show(ctx, |ui| {
                self.ingredients_panel(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.plates_panel(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Gestion des Coûts F&B propulsé par ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(".");
                });
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

# Scullery

Sovereign recipe system. Ingredients flow through transformations.

Scullery is a data-driven Rust CLI for managing historically-researched recipes with full nutritional analysis and real supplier sourcing. Ingredients are data. Processing steps are transformations. No OOP, no central state, just pure composition from raw grain to finished meal.

**[wildernessinteractive.com](https://wildernessinteractive.com)**

## Architecture

```
Seed Data --> Processing Steps --> Cooking Instructions
(ingredients,   (mill, crack,      (portions, macros,
 sourcing)       dehydrate)         larder status)
```

- **Data models**: Form, Ingredient, Source, Process, Container
- **Pure functions**: Seed, process, cook, display
- **No runtime**: CLI output, no server, no framework

## Current Recipes

- **Skog Grautr** - Old Norse forest porridge (cracked wheat, barley, rye, oats, porcini mushrooms, herbs)
- **Puls Fabata** - Ancient Roman bean and grain porridge (emmer, fava beans, pork fat)

Both historically sourced from primary texts (Ovid, Pliny, Cato, Apicius) with full nutritional breakdown per serving.

## Features

- Nutritional analysis (macros per 100g, per serving, per meal)
- Real UK supplier sourcing with current prices
- Processing step tracking (mortarium, hand mill, dehydrate, render)
- Larder inventory status
- Historical documentation with primary source citations

## Setup

### Build

```
cargo build
```

### Run

```
cargo run
```

## License

Wilderness Interactive Open License

Permission is hereby granted, free of charge, to use, copy, modify, and distribute this software for any purpose, including commercial use.

This software may NOT be:
- Sold as a standalone product
- Sold access to as a hosted service

Use for building software, building websites, automating workflows, and integrating with other tools (including commercial work) is explicitly permitted and encouraged. This software is designed to be moddable, so modifications are explicitly permitted and encouraged. Software and systems built using this tool can be sold freely.

The purpose of this license is to prevent reselling the software itself.

---

Built by [Wilderness Interactive](https://wildernessinteractive.com).
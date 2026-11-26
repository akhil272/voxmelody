import { ALL, MAX_MODEL_BYTES } from "#lib/constants.js";
import type { ModelFilter, ModelInfo, ModelSort } from "#lib/models/types.js";

function compare(sort: ModelSort, sizeOf: (id: string) => number) {
  return (a: ModelInfo, b: ModelInfo) => {
    if (sort === "newest") return b.released.localeCompare(a.released);
    if (sort === "smallest") return sizeOf(a.id) - sizeOf(b.id);
    if (sort === "name") return a.name.localeCompare(b.name);
    // The catalog already lists recommended models first.
    return 0;
  };
}

export function filterModels(
  models: ModelInfo[],
  filter: ModelFilter,
  sizeOf: (id: string) => number,
  isFavourite: (id: string) => boolean,
) {
  const search = filter.search.trim().toLowerCase();
  return models
    .filter(
      (model) =>
        `${model.name} ${model.author}`.toLowerCase().includes(search) &&
        (filter.kind === ALL || model.kind === filter.kind) &&
        (filter.licence === ALL || model.licence === filter.licence) &&
        sizeOf(model.id) < MAX_MODEL_BYTES[filter.size] &&
        (!filter.favourites || isFavourite(model.id)),
    )
    .sort(compare(filter.sort, sizeOf));
}

export function toModelOptions(models: ModelInfo[]) {
  return models.map(({ id, name }) => ({ value: id, label: name }));
}

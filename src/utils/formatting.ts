export function formatCategoryName(name: string): string {
  if (!name) return '';

  const replacements: Record<string, string> = {
    '2d-asset': '2D Asset',
    '3d-model': '3D Model',
    'education-tutorial': 'Education / Tutorial',
    'game-system': 'Game System',
    'game-template': 'Game Template',
    hdri: 'HDRI',
    'smart-asset': 'Smart Asset',
    'tool-and-plugin': 'Tool and Plugin',
    ui: 'UI',
    vfx: 'VFX',
  };

  return replacements[name.toLowerCase()] || name.charAt(0).toUpperCase() + name.slice(1).replace(/-/g, ' ');
}

export function unformatCategoryName(formattedName: string): string {
  if (!formattedName) return '';

  const reverseReplacements: Record<string, string> = {
    '2D Asset': '2d-asset',
    '3D Model': '3d-model',
    'Education / Tutorial': 'education-tutorial',
    'Game Systems': 'game-system',
    'Game Templates': 'game-template',
    'HDRI': 'hdri',
    'Smart Asset': 'smart-asset',
    'Tools & Plugins': 'tool-and-plugin',
    'UI': 'ui',
    'VFX': 'vfx',
  };

  // First, check the direct reverse replacements
  const slug = reverseReplacements[formattedName];
  if (slug) {
    return slug;
  }

  // Fallback for capitalized words that are not in the map
  if (formattedName === 'Animation' || formattedName === 'Audio' || formattedName === 'Environment' || formattedName === 'Material') {
    return formattedName.toLowerCase();
  }

  // Fallback for other cases - this is a simple version
  return formattedName.toLowerCase().replace(/ /g, '-');
} 
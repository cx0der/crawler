import 'vuetify/styles'
import { createVuetify, type ThemeDefinition } from 'vuetify'
import { aliases, mdi } from 'vuetify/iconsets/mdi'
import { md3 } from 'vuetify/blueprints'

const lightThemeDef: ThemeDefinition = {
  dark: false,
  colors: {
    background: '#FFFBFE',
    surface: '#FFFBFE',
    primary: '#D0BCFF',
    secondary: '#625B71',
    error: '#B3261E',
    'on-background': '#1C1B1F',
    'on-surface': '#1C1B1F',
    'on-primary': '#FFFFFF',
    'on-secondary': '#FFFFFF',
    'on-error': '#FFFFFF'
  }
}
const darkThemeDef: ThemeDefinition = {
  dark: true,
  colors: {
    background: '#1C1B1F',
    surface: '#1C1B1F',
    primary: '#D0BCFF',
    secondary: '#CCC2DC',
    error: '#F2B8B5',
    'on-background': '#E6E1E5',
    'on-surface': '#E6E1E5',
    'on-primary': '#381E72',
    'on-secondary': '#332D41',
    'on-error': '#601410'
  }
}

export default createVuetify({
  blueprint: md3,
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi
    }
  },
  theme: {
    themes: {
      lightThemeDef,
      darkThemeDef
    }
  }
})

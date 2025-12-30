import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/path";

export type Settings = {
  max_backups: number;
  parallel_requests: number;
};

export type Font = {
  name: string;
  path: string;
};

export type Theme = {
  name: string;
  path: string;
};

export interface SettingsState {
  settings: Settings;
  loading: boolean;
  fonts: Font[];
  fontsLoading: boolean;
  currentFont: Font | null;
  currentFontLoading: boolean;
  themes: Theme[];
  themesLoading: boolean;
  currentTheme: Theme | null;
  currentThemeLoading: boolean;
}

const initialState: SettingsState = {
  settings: {
    max_backups: 5,
    parallel_requests: 4,
  },
  loading: true,
  fonts: [],
  fontsLoading: true,
  currentFont: null,
  currentFontLoading: true,
  themes: [],
  themesLoading: true,
  currentTheme: null,
  currentThemeLoading: true,
};

export const getSettings = createAsyncThunk(
  "settings/getSettings",
  async () => {
    const settings = await invoke<Settings>("get_settings");
    return settings;
  }
);

export const getMonospaceFonts = createAsyncThunk(
  "settings/getMonospaceFonts",
  async () => {
    const fonts = await invoke<Font[]>("get_monospace_fonts");
    return fonts;
  }
);

export const getCurrentFont = createAsyncThunk(
  "settings/getCurrentFont",
  async () => {
    const font = await invoke<Font | null>("get_current_font");
    return font;
  }
);

export const applyFont = createAsyncThunk(
  "settings/applyFont",
  async (font: Font) => {
    await invoke("apply_font", { fontName: font.name, fontPath: font.path });
  }
);

export const getAvailableThemes = createAsyncThunk(
  "settings/getAvailableThemes",
  async () => {
    const themes = await invoke<Theme[]>("get_available_themes");
    return themes;
  }
);

export const getCurrentTheme = createAsyncThunk(
  "settings/getCurrentTheme",
  async () => {
    const theme = await invoke<Theme | null>("get_current_theme");
    return theme;
  }
);

export const applyTheme = createAsyncThunk(
  "settings/applyTheme",
  async (theme: Theme) => {
    await invoke("apply_theme", { themePath: theme.path });
  }
);

export const settingsSlice = createSlice({
  name: "settings",
  initialState,
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(getSettings.pending, (state) => {
        state.loading = true;
      })
      .addCase(getSettings.fulfilled, (state, action) => {
        state.settings = action.payload;
        state.loading = false;
      })
      .addCase(getSettings.rejected, (state) => {
        state.loading = false;
      })
      .addCase(getMonospaceFonts.pending, (state) => {
        state.fontsLoading = true;
      })
      .addCase(getMonospaceFonts.fulfilled, (state, action) => {
        state.fonts = action.payload;
        state.fontsLoading = false;
      })
      .addCase(getMonospaceFonts.rejected, (state) => {
        state.fontsLoading = false;
      })
      .addCase(getCurrentFont.pending, (state) => {
        state.currentFontLoading = true;
      })
      .addCase(getCurrentFont.fulfilled, (state, action) => {
        state.currentFont = action.payload;
        state.currentFontLoading = false;
      })
      .addCase(getCurrentFont.rejected, (state) => {
        state.currentFontLoading = false;
      })
      .addCase(getAvailableThemes.pending, (state) => {
        state.themesLoading = true;
      })
      .addCase(getAvailableThemes.fulfilled, (state, action) => {
        state.themes = action.payload;
        state.themesLoading = false;
      })
      .addCase(getAvailableThemes.rejected, (state) => {
        state.themesLoading = false;
      })
      .addCase(getCurrentTheme.pending, (state) => {
        state.currentThemeLoading = true;
      })
      .addCase(getCurrentTheme.fulfilled, (state, action) => {
        state.currentTheme = action.payload;
        state.currentThemeLoading = false;
      })
      .addCase(getCurrentTheme.rejected, (state) => {
        state.currentThemeLoading = false;
      });
  },
});

export const { reducer: settingsReducer } = settingsSlice;

export const selectSettings = (state: { settings: SettingsState }) =>
  state.settings.settings;

export const selectFonts = (state: { settings: SettingsState }) =>
  state.settings.fonts;

export const selectCurrentFont = (state: { settings: SettingsState }) =>
  state.settings.currentFont;

export const selectThemes = (state: { settings: SettingsState }) =>
  state.settings.themes;

export const selectCurrentTheme = (state: { settings: SettingsState }) =>
  state.settings.currentTheme;

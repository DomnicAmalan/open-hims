'use strict';

var reactNativePaper = require('react-native-paper');
var reactNative = require('react-native');
var jsxRuntime = require('react/jsx-runtime');

var __defProp = Object.defineProperty;
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __publicField = (obj, key, value) => __defNormalProp(obj, key + "" , value);
var himsLightTheme = {
  ...reactNativePaper.MD3LightTheme,
  colors: {
    ...reactNativePaper.MD3LightTheme.colors,
    primary: "#0066cc",
    primaryContainer: "#e6f2ff",
    onPrimary: "#ffffff",
    onPrimaryContainer: "#001429",
    secondary: "#4caf50",
    secondaryContainer: "#e8f5e8",
    onSecondary: "#ffffff",
    onSecondaryContainer: "#1b5e20",
    error: "#f44336",
    errorContainer: "#ffebee",
    onError: "#ffffff",
    onErrorContainer: "#b71c1c",
    surface: "#ffffff",
    onSurface: "#1c1b1f",
    surfaceVariant: "#f5f5f5",
    onSurfaceVariant: "#49454f",
    outline: "#d1d5db",
    background: "#fafafa",
    onBackground: "#1c1b1f"
  }
};
var himsDarkTheme = {
  ...reactNativePaper.MD3DarkTheme,
  colors: {
    ...reactNativePaper.MD3DarkTheme.colors,
    primary: "#4da6ff",
    primaryContainer: "#003d7a",
    onPrimary: "#001429",
    onPrimaryContainer: "#e6f2ff",
    secondary: "#81c784",
    secondaryContainer: "#2e7d32",
    onSecondary: "#1b5e20",
    onSecondaryContainer: "#e8f5e8",
    error: "#ef5350",
    errorContainer: "#d32f2f",
    onError: "#b71c1c",
    onErrorContainer: "#ffebee",
    surface: "#1c1b1f",
    onSurface: "#e6e1e5",
    surfaceVariant: "#49454f",
    onSurfaceVariant: "#cac4d0",
    outline: "#938f99",
    background: "#121212",
    onBackground: "#e6e1e5"
  }
};
function HimsPaperProvider({ children, theme = "auto" }) {
  const systemColorScheme = reactNative.useColorScheme();
  const selectedTheme = theme === "auto" ? systemColorScheme === "dark" ? himsDarkTheme : himsLightTheme : theme === "dark" ? himsDarkTheme : himsLightTheme;
  return /* @__PURE__ */ jsxRuntime.jsx(reactNativePaper.PaperProvider, { theme: selectedTheme, children });
}
var LINKING_ERROR = `The package 'hims-core-sdk-react-native' doesn't seem to be linked. Make sure: 

` + reactNative.Platform.select({ ios: "- Run 'cd ios && pod install'\n", default: "" }) + "- You rebuilt the app after installing the package\n- You are not using Expo Go\n";
var HimsCoreSdk = reactNative.NativeModules.HimsCoreSdk ? reactNative.NativeModules.HimsCoreSdk : new Proxy(
  {},
  {
    get() {
      throw new Error(LINKING_ERROR);
    }
  }
);
var HimsCoreSDK = class {
  constructor(config) {
    this.config = config;
    __publicField(this, "core", null);
  }
  async initialize() {
    if (!this.core) {
      this.core = await HimsCoreSdk.createHimsCore(this.config);
    }
    return this.core.initialize();
  }
  // FHIR Methods
  async createPatient(patientData) {
    return HimsCoreSdk.createPatient(patientData);
  }
  async getPatient(id) {
    return HimsCoreSdk.getPatient(id);
  }
  // HL7v2 Methods
  async parseHL7Message(message) {
    return HimsCoreSdk.parseHL7Message(message);
  }
  // DICOM Methods
  async parseDicomMetadata(file) {
    return HimsCoreSdk.parseDicomMetadata(file);
  }
  // ABDM Methods
  async initiateConsent(request) {
    return HimsCoreSdk.initiateConsent(request);
  }
};

Object.defineProperty(exports, "ActivityIndicator", {
  enumerable: true,
  get: function () { return reactNativePaper.ActivityIndicator; }
});
Object.defineProperty(exports, "Appbar", {
  enumerable: true,
  get: function () { return reactNativePaper.Appbar; }
});
Object.defineProperty(exports, "Avatar", {
  enumerable: true,
  get: function () { return reactNativePaper.Avatar; }
});
Object.defineProperty(exports, "Badge", {
  enumerable: true,
  get: function () { return reactNativePaper.Badge; }
});
Object.defineProperty(exports, "Banner", {
  enumerable: true,
  get: function () { return reactNativePaper.Banner; }
});
Object.defineProperty(exports, "BottomNavigation", {
  enumerable: true,
  get: function () { return reactNativePaper.BottomNavigation; }
});
Object.defineProperty(exports, "Button", {
  enumerable: true,
  get: function () { return reactNativePaper.Button; }
});
Object.defineProperty(exports, "Card", {
  enumerable: true,
  get: function () { return reactNativePaper.Card; }
});
Object.defineProperty(exports, "Checkbox", {
  enumerable: true,
  get: function () { return reactNativePaper.Checkbox; }
});
Object.defineProperty(exports, "Chip", {
  enumerable: true,
  get: function () { return reactNativePaper.Chip; }
});
Object.defineProperty(exports, "DataTable", {
  enumerable: true,
  get: function () { return reactNativePaper.DataTable; }
});
Object.defineProperty(exports, "Dialog", {
  enumerable: true,
  get: function () { return reactNativePaper.Dialog; }
});
Object.defineProperty(exports, "Divider", {
  enumerable: true,
  get: function () { return reactNativePaper.Divider; }
});
Object.defineProperty(exports, "FAB", {
  enumerable: true,
  get: function () { return reactNativePaper.FAB; }
});
Object.defineProperty(exports, "IconButton", {
  enumerable: true,
  get: function () { return reactNativePaper.IconButton; }
});
Object.defineProperty(exports, "List", {
  enumerable: true,
  get: function () { return reactNativePaper.List; }
});
Object.defineProperty(exports, "Menu", {
  enumerable: true,
  get: function () { return reactNativePaper.Menu; }
});
Object.defineProperty(exports, "Portal", {
  enumerable: true,
  get: function () { return reactNativePaper.Portal; }
});
Object.defineProperty(exports, "ProgressBar", {
  enumerable: true,
  get: function () { return reactNativePaper.ProgressBar; }
});
Object.defineProperty(exports, "RadioButton", {
  enumerable: true,
  get: function () { return reactNativePaper.RadioButton; }
});
Object.defineProperty(exports, "Searchbar", {
  enumerable: true,
  get: function () { return reactNativePaper.Searchbar; }
});
Object.defineProperty(exports, "SegmentedButtons", {
  enumerable: true,
  get: function () { return reactNativePaper.SegmentedButtons; }
});
Object.defineProperty(exports, "Snackbar", {
  enumerable: true,
  get: function () { return reactNativePaper.Snackbar; }
});
Object.defineProperty(exports, "Surface", {
  enumerable: true,
  get: function () { return reactNativePaper.Surface; }
});
Object.defineProperty(exports, "Switch", {
  enumerable: true,
  get: function () { return reactNativePaper.Switch; }
});
Object.defineProperty(exports, "TextInput", {
  enumerable: true,
  get: function () { return reactNativePaper.TextInput; }
});
exports.HimsCoreSDK = HimsCoreSDK;
exports.HimsPaperProvider = HimsPaperProvider;
//# sourceMappingURL=index.js.map
//# sourceMappingURL=index.js.map
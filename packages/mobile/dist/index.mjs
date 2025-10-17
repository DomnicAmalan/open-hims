import { MD3LightTheme, MD3DarkTheme, PaperProvider } from 'react-native-paper';
export { ActivityIndicator, Appbar, Avatar, Badge, Banner, BottomNavigation, Button, Card, Checkbox, Chip, DataTable, Dialog, Divider, FAB, IconButton, List, Menu, Portal, ProgressBar, RadioButton, Searchbar, SegmentedButtons, Snackbar, Surface, Switch, TextInput } from 'react-native-paper';
import { Platform, NativeModules, useColorScheme } from 'react-native';
import { jsx } from 'react/jsx-runtime';

var __defProp = Object.defineProperty;
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __publicField = (obj, key, value) => __defNormalProp(obj, key + "" , value);
var himsLightTheme = {
  ...MD3LightTheme,
  colors: {
    ...MD3LightTheme.colors,
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
  ...MD3DarkTheme,
  colors: {
    ...MD3DarkTheme.colors,
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
  const systemColorScheme = useColorScheme();
  const selectedTheme = theme === "auto" ? systemColorScheme === "dark" ? himsDarkTheme : himsLightTheme : theme === "dark" ? himsDarkTheme : himsLightTheme;
  return /* @__PURE__ */ jsx(PaperProvider, { theme: selectedTheme, children });
}
var LINKING_ERROR = `The package 'hims-core-sdk-react-native' doesn't seem to be linked. Make sure: 

` + Platform.select({ ios: "- Run 'cd ios && pod install'\n", default: "" }) + "- You rebuilt the app after installing the package\n- You are not using Expo Go\n";
var HimsCoreSdk = NativeModules.HimsCoreSdk ? NativeModules.HimsCoreSdk : new Proxy(
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

export { HimsCoreSDK, HimsPaperProvider };
//# sourceMappingURL=index.mjs.map
//# sourceMappingURL=index.mjs.map
'use strict';

var core = require('@mantine/core');
var notifications = require('@mantine/notifications');
var modals = require('@mantine/modals');
var jsxRuntime = require('react/jsx-runtime');
var hooks = require('@mantine/hooks');
var form = require('@mantine/form');

// src/components/theme-provider.tsx
var himsBlue = [
  "#e6f2ff",
  "#b3d9ff",
  "#80bfff",
  "#4da6ff",
  "#1a8cff",
  "#0066cc",
  // Primary
  "#0052a3",
  "#003d7a",
  "#002952",
  "#001429"
];
var himsGreen = [
  "#e8f5e8",
  "#c8e6c9",
  "#a5d6a7",
  "#81c784",
  "#66bb6a",
  "#4caf50",
  // Success
  "#43a047",
  "#388e3c",
  "#2e7d32",
  "#1b5e20"
];
var himsRed = [
  "#ffebee",
  "#ffcdd2",
  "#ef9a9a",
  "#e57373",
  "#ef5350",
  "#f44336",
  // Error
  "#e53935",
  "#d32f2f",
  "#c62828",
  "#b71c1c"
];
var theme = core.createTheme({
  primaryColor: "himsBlue",
  colors: {
    himsBlue,
    himsGreen,
    himsRed
  },
  fontFamily: "Inter, system-ui, -apple-system, sans-serif",
  headings: {
    fontFamily: "Inter, system-ui, -apple-system, sans-serif",
    fontWeight: "600"
  },
  defaultRadius: "md",
  components: {
    Button: {
      defaultProps: {
        radius: "md"
      }
    },
    Card: {
      defaultProps: {
        shadow: "sm",
        radius: "md",
        withBorder: true
      }
    },
    TextInput: {
      defaultProps: {
        radius: "md"
      }
    },
    Select: {
      defaultProps: {
        radius: "md"
      }
    }
  },
  other: {
    // Healthcare-specific theme values
    spacing: {
      xs: "0.5rem",
      sm: "0.75rem",
      md: "1rem",
      lg: "1.5rem",
      xl: "2rem"
    },
    // Accessibility improvements
    focusRing: "always",
    respectReducedMotion: true
  }
});
function HimsThemeProvider({ children }) {
  return /* @__PURE__ */ jsxRuntime.jsxs(core.MantineProvider, { theme, defaultColorScheme: "light", children: [
    /* @__PURE__ */ jsxRuntime.jsx(notifications.Notifications, { position: "top-right", zIndex: 2077 }),
    /* @__PURE__ */ jsxRuntime.jsx(modals.ModalsProvider, { children })
  ] });
}

Object.defineProperty(exports, "Accordion", {
  enumerable: true,
  get: function () { return core.Accordion; }
});
Object.defineProperty(exports, "ActionIcon", {
  enumerable: true,
  get: function () { return core.ActionIcon; }
});
Object.defineProperty(exports, "Alert", {
  enumerable: true,
  get: function () { return core.Alert; }
});
Object.defineProperty(exports, "AppShell", {
  enumerable: true,
  get: function () { return core.AppShell; }
});
Object.defineProperty(exports, "Autocomplete", {
  enumerable: true,
  get: function () { return core.Autocomplete; }
});
Object.defineProperty(exports, "Avatar", {
  enumerable: true,
  get: function () { return core.Avatar; }
});
Object.defineProperty(exports, "Badge", {
  enumerable: true,
  get: function () { return core.Badge; }
});
Object.defineProperty(exports, "Button", {
  enumerable: true,
  get: function () { return core.Button; }
});
Object.defineProperty(exports, "Card", {
  enumerable: true,
  get: function () { return core.Card; }
});
Object.defineProperty(exports, "Checkbox", {
  enumerable: true,
  get: function () { return core.Checkbox; }
});
Object.defineProperty(exports, "ColorInput", {
  enumerable: true,
  get: function () { return core.ColorInput; }
});
Object.defineProperty(exports, "Combobox", {
  enumerable: true,
  get: function () { return core.Combobox; }
});
Object.defineProperty(exports, "Container", {
  enumerable: true,
  get: function () { return core.Container; }
});
Object.defineProperty(exports, "Divider", {
  enumerable: true,
  get: function () { return core.Divider; }
});
Object.defineProperty(exports, "Drawer", {
  enumerable: true,
  get: function () { return core.Drawer; }
});
Object.defineProperty(exports, "FileInput", {
  enumerable: true,
  get: function () { return core.FileInput; }
});
Object.defineProperty(exports, "Grid", {
  enumerable: true,
  get: function () { return core.Grid; }
});
Object.defineProperty(exports, "Group", {
  enumerable: true,
  get: function () { return core.Group; }
});
Object.defineProperty(exports, "JsonInput", {
  enumerable: true,
  get: function () { return core.JsonInput; }
});
Object.defineProperty(exports, "Loader", {
  enumerable: true,
  get: function () { return core.Loader; }
});
Object.defineProperty(exports, "Menu", {
  enumerable: true,
  get: function () { return core.Menu; }
});
Object.defineProperty(exports, "Modal", {
  enumerable: true,
  get: function () { return core.Modal; }
});
Object.defineProperty(exports, "MultiSelect", {
  enumerable: true,
  get: function () { return core.MultiSelect; }
});
Object.defineProperty(exports, "Notification", {
  enumerable: true,
  get: function () { return core.Notification; }
});
Object.defineProperty(exports, "NumberInput", {
  enumerable: true,
  get: function () { return core.NumberInput; }
});
Object.defineProperty(exports, "Paper", {
  enumerable: true,
  get: function () { return core.Paper; }
});
Object.defineProperty(exports, "PasswordInput", {
  enumerable: true,
  get: function () { return core.PasswordInput; }
});
Object.defineProperty(exports, "PinInput", {
  enumerable: true,
  get: function () { return core.PinInput; }
});
Object.defineProperty(exports, "Progress", {
  enumerable: true,
  get: function () { return core.Progress; }
});
Object.defineProperty(exports, "Radio", {
  enumerable: true,
  get: function () { return core.Radio; }
});
Object.defineProperty(exports, "Select", {
  enumerable: true,
  get: function () { return core.Select; }
});
Object.defineProperty(exports, "Stack", {
  enumerable: true,
  get: function () { return core.Stack; }
});
Object.defineProperty(exports, "Stepper", {
  enumerable: true,
  get: function () { return core.Stepper; }
});
Object.defineProperty(exports, "Switch", {
  enumerable: true,
  get: function () { return core.Switch; }
});
Object.defineProperty(exports, "Table", {
  enumerable: true,
  get: function () { return core.Table; }
});
Object.defineProperty(exports, "Tabs", {
  enumerable: true,
  get: function () { return core.Tabs; }
});
Object.defineProperty(exports, "TagsInput", {
  enumerable: true,
  get: function () { return core.TagsInput; }
});
Object.defineProperty(exports, "TextInput", {
  enumerable: true,
  get: function () { return core.TextInput; }
});
Object.defineProperty(exports, "Textarea", {
  enumerable: true,
  get: function () { return core.Textarea; }
});
Object.defineProperty(exports, "Timeline", {
  enumerable: true,
  get: function () { return core.Timeline; }
});
Object.defineProperty(exports, "Tooltip", {
  enumerable: true,
  get: function () { return core.Tooltip; }
});
Object.defineProperty(exports, "cleanNotifications", {
  enumerable: true,
  get: function () { return notifications.cleanNotifications; }
});
Object.defineProperty(exports, "hideNotification", {
  enumerable: true,
  get: function () { return notifications.hideNotification; }
});
Object.defineProperty(exports, "notifications", {
  enumerable: true,
  get: function () { return notifications.notifications; }
});
Object.defineProperty(exports, "showNotification", {
  enumerable: true,
  get: function () { return notifications.showNotification; }
});
Object.defineProperty(exports, "updateNotification", {
  enumerable: true,
  get: function () { return notifications.updateNotification; }
});
Object.defineProperty(exports, "closeAllModals", {
  enumerable: true,
  get: function () { return modals.closeAllModals; }
});
Object.defineProperty(exports, "closeModal", {
  enumerable: true,
  get: function () { return modals.closeModal; }
});
Object.defineProperty(exports, "modals", {
  enumerable: true,
  get: function () { return modals.modals; }
});
Object.defineProperty(exports, "openConfirmModal", {
  enumerable: true,
  get: function () { return modals.openConfirmModal; }
});
Object.defineProperty(exports, "openContextModal", {
  enumerable: true,
  get: function () { return modals.openContextModal; }
});
Object.defineProperty(exports, "openModal", {
  enumerable: true,
  get: function () { return modals.openModal; }
});
Object.defineProperty(exports, "useClickOutside", {
  enumerable: true,
  get: function () { return hooks.useClickOutside; }
});
Object.defineProperty(exports, "useCounter", {
  enumerable: true,
  get: function () { return hooks.useCounter; }
});
Object.defineProperty(exports, "useDisclosure", {
  enumerable: true,
  get: function () { return hooks.useDisclosure; }
});
Object.defineProperty(exports, "useDocumentTitle", {
  enumerable: true,
  get: function () { return hooks.useDocumentTitle; }
});
Object.defineProperty(exports, "useFocusTrap", {
  enumerable: true,
  get: function () { return hooks.useFocusTrap; }
});
Object.defineProperty(exports, "useHotkeys", {
  enumerable: true,
  get: function () { return hooks.useHotkeys; }
});
Object.defineProperty(exports, "useIdle", {
  enumerable: true,
  get: function () { return hooks.useIdle; }
});
Object.defineProperty(exports, "useInputState", {
  enumerable: true,
  get: function () { return hooks.useInputState; }
});
Object.defineProperty(exports, "useInterval", {
  enumerable: true,
  get: function () { return hooks.useInterval; }
});
Object.defineProperty(exports, "useLocalStorage", {
  enumerable: true,
  get: function () { return hooks.useLocalStorage; }
});
Object.defineProperty(exports, "useNetwork", {
  enumerable: true,
  get: function () { return hooks.useNetwork; }
});
Object.defineProperty(exports, "useOs", {
  enumerable: true,
  get: function () { return hooks.useOs; }
});
Object.defineProperty(exports, "usePrevious", {
  enumerable: true,
  get: function () { return hooks.usePrevious; }
});
Object.defineProperty(exports, "useQueue", {
  enumerable: true,
  get: function () { return hooks.useQueue; }
});
Object.defineProperty(exports, "useSessionStorage", {
  enumerable: true,
  get: function () { return hooks.useSessionStorage; }
});
Object.defineProperty(exports, "useSetState", {
  enumerable: true,
  get: function () { return hooks.useSetState; }
});
Object.defineProperty(exports, "useTimeout", {
  enumerable: true,
  get: function () { return hooks.useTimeout; }
});
Object.defineProperty(exports, "useToggle", {
  enumerable: true,
  get: function () { return hooks.useToggle; }
});
Object.defineProperty(exports, "useUncontrolled", {
  enumerable: true,
  get: function () { return hooks.useUncontrolled; }
});
Object.defineProperty(exports, "useValidatedState", {
  enumerable: true,
  get: function () { return hooks.useValidatedState; }
});
Object.defineProperty(exports, "useViewportSize", {
  enumerable: true,
  get: function () { return hooks.useViewportSize; }
});
Object.defineProperty(exports, "useWindowScroll", {
  enumerable: true,
  get: function () { return hooks.useWindowScroll; }
});
Object.defineProperty(exports, "hasLength", {
  enumerable: true,
  get: function () { return form.hasLength; }
});
Object.defineProperty(exports, "isEmail", {
  enumerable: true,
  get: function () { return form.isEmail; }
});
Object.defineProperty(exports, "isInRange", {
  enumerable: true,
  get: function () { return form.isInRange; }
});
Object.defineProperty(exports, "isNotEmpty", {
  enumerable: true,
  get: function () { return form.isNotEmpty; }
});
Object.defineProperty(exports, "matches", {
  enumerable: true,
  get: function () { return form.matches; }
});
Object.defineProperty(exports, "useForm", {
  enumerable: true,
  get: function () { return form.useForm; }
});
exports.HimsThemeProvider = HimsThemeProvider;
//# sourceMappingURL=index.js.map
//# sourceMappingURL=index.js.map
import { createTheme, MantineProvider } from '@mantine/core';
export { Accordion, ActionIcon, Alert, AppShell, Autocomplete, Avatar, Badge, Button, Card, Checkbox, ColorInput, Combobox, Container, Divider, Drawer, FileInput, Grid, Group, JsonInput, Loader, Menu, Modal, MultiSelect, Notification, NumberInput, Paper, PasswordInput, PinInput, Progress, Radio, Select, Stack, Stepper, Switch, Table, Tabs, TagsInput, TextInput, Textarea, Timeline, Tooltip } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
export { cleanNotifications, hideNotification, notifications, showNotification, updateNotification } from '@mantine/notifications';
import { ModalsProvider } from '@mantine/modals';
export { closeAllModals, closeModal, modals, openConfirmModal, openContextModal, openModal } from '@mantine/modals';
import { jsxs, jsx } from 'react/jsx-runtime';
export { useClickOutside, useCounter, useDisclosure, useDocumentTitle, useFocusTrap, useHotkeys, useIdle, useInputState, useInterval, useLocalStorage, useNetwork, useOs, usePrevious, useQueue, useSessionStorage, useSetState, useTimeout, useToggle, useUncontrolled, useValidatedState, useViewportSize, useWindowScroll } from '@mantine/hooks';
export { hasLength, isEmail, isInRange, isNotEmpty, matches, useForm } from '@mantine/form';

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
var theme = createTheme({
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
  return /* @__PURE__ */ jsxs(MantineProvider, { theme, defaultColorScheme: "light", children: [
    /* @__PURE__ */ jsx(Notifications, { position: "top-right", zIndex: 2077 }),
    /* @__PURE__ */ jsx(ModalsProvider, { children })
  ] });
}

export { HimsThemeProvider };
//# sourceMappingURL=index.mjs.map
//# sourceMappingURL=index.mjs.map
// Mantine UI Components and Theme Provider
export * from './components/theme-provider';
export { theme, HimsThemeProvider } from './components/theme-provider';

// Re-export Mantine components for convenience
export {
  AppShell,
  Button,
  Card,
  TextInput,
  Select,
  MultiSelect,
  Avatar,
  Table,
  Modal,
  Drawer,
  Tooltip,
  Notification,
  Badge,
  Group,
  Stack,
  Grid,
  Container,
  Paper,
  Divider,
  ActionIcon,
  Loader,
  Alert,
  Menu,
  Switch,
  Radio,
  Checkbox,
  Progress,
  Stepper,
  Tabs,
  Accordion,
  Timeline,
  NumberInput,
  Textarea,
  PasswordInput,
  FileInput,
  ColorInput,
  JsonInput,
  Autocomplete,
  Combobox,
  TagsInput,
  PinInput,
} from '@mantine/core';

// Mantine hooks
export {
  useDisclosure,
  useToggle,
  useCounter,
  useInputState,
  useLocalStorage,
  useSessionStorage,
  useViewportSize,
  useDocumentTitle,
  useFocusTrap,
  useClickOutside,
  useHotkeys,
  useInterval,
  useTimeout,
  useIdle,
  useNetwork,
  useOs,
  usePrevious,
  useQueue,
  useSetState,
  useUncontrolled,
  useValidatedState,
  useWindowScroll,
} from '@mantine/hooks';

// Mantine forms
export {
  useForm,
  isEmail,
  isNotEmpty,
  hasLength,
  matches,
  isInRange,
} from '@mantine/form';

// Mantine notifications
export {
  notifications,
  showNotification,
  hideNotification,
  cleanNotifications,
  updateNotification,
} from '@mantine/notifications';

// Mantine modals
export {
  modals,
  openModal,
  closeModal,
  closeAllModals,
  openConfirmModal,
  openContextModal,
} from '@mantine/modals';

// Version information
export const WEB_VERSION = '1.0.0';
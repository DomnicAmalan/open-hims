// React Native Paper UI Components
export * from './providers/theme-provider';

// Rust Core SDK Bindings for React Native
export * from './bindings';

// Re-export essential React Native components
export { View, ScrollView, FlatList, SectionList } from 'react-native';

// Re-export React Native Paper for convenience
export {
  Button,
  Card,
  TextInput,
  Avatar,
  List,
  FAB,
  Dialog,
  Portal,
  DataTable,
  Chip,
  Badge,
  Surface,
  Divider,
  IconButton,
  ActivityIndicator,
  Snackbar,
  Menu,
  Switch,
  RadioButton,
  Checkbox,
  ProgressBar,
  Banner,
  Appbar,
  BottomNavigation,
  Searchbar,
  SegmentedButtons,
  Text,
  TouchableRipple,
} from 'react-native-paper';

// Version information
export const MOBILE_VERSION = '1.0.0';
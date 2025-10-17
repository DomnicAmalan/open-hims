import * as react_jsx_runtime from 'react/jsx-runtime';
import React from 'react';
export { Accordion, ActionIcon, Alert, AppShell, Autocomplete, Avatar, Badge, Button, Card, Checkbox, ColorInput, Combobox, Container, Divider, Drawer, FileInput, Grid, Group, JsonInput, Loader, Menu, Modal, MultiSelect, Notification, NumberInput, Paper, PasswordInput, PinInput, Progress, Radio, Select, Stack, Stepper, Switch, Table, Tabs, TagsInput, TextInput, Textarea, Timeline, Tooltip } from '@mantine/core';
export { useClickOutside, useCounter, useDisclosure, useDocumentTitle, useFocusTrap, useHotkeys, useIdle, useInputState, useInterval, useLocalStorage, useNetwork, useOs, usePrevious, useQueue, useSessionStorage, useSetState, useTimeout, useToggle, useUncontrolled, useValidatedState, useViewportSize, useWindowScroll } from '@mantine/hooks';
export { hasLength, isEmail, isInRange, isNotEmpty, matches, useForm } from '@mantine/form';
export { cleanNotifications, hideNotification, notifications, showNotification, updateNotification } from '@mantine/notifications';
export { closeAllModals, closeModal, modals, openConfirmModal, openContextModal, openModal } from '@mantine/modals';

interface HimsThemeProviderProps {
    children: React.ReactNode;
}
declare function HimsThemeProvider({ children }: HimsThemeProviderProps): react_jsx_runtime.JSX.Element;

export { HimsThemeProvider };

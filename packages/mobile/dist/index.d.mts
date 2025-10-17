import * as react_jsx_runtime from 'react/jsx-runtime';
import React from 'react';
export { ActivityIndicator, Appbar, Avatar, Badge, Banner, BottomNavigation, Button, Card, Checkbox, Chip, DataTable, Dialog, Divider, FAB, IconButton, List, Menu, Portal, ProgressBar, RadioButton, Searchbar, SegmentedButtons, Snackbar, Surface, Switch, TextInput } from 'react-native-paper';

interface HimsPaperProviderProps {
    children: React.ReactNode;
    theme?: 'light' | 'dark' | 'auto';
}
declare function HimsPaperProvider({ children, theme }: HimsPaperProviderProps): react_jsx_runtime.JSX.Element;

interface HimsConfig {
    apiEndpoint: string;
    authToken?: string;
    enableLogging: boolean;
}
interface HimsCore {
    initialize(): Promise<string>;
}
declare class HimsCoreSDK {
    private config;
    private core;
    constructor(config: HimsConfig);
    initialize(): Promise<string>;
    createPatient(patientData: any): Promise<any>;
    getPatient(id: string): Promise<any>;
    parseHL7Message(message: string): Promise<any>;
    parseDicomMetadata(file: string): Promise<any>;
    initiateConsent(request: any): Promise<any>;
}

export { type HimsConfig, type HimsCore, HimsCoreSDK, HimsPaperProvider };

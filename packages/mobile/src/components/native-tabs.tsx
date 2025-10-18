import React from 'react';
import { MaterialCommunityIcons } from '@expo/vector-icons';

// Preset Tab Icons for Healthcare - Compatible with Expo Router

export const HimsTabIcons = {
  dashboard: (props: { focused: boolean; color: string; size: number }) => (
    <MaterialCommunityIcons 
      name={props.focused ? 'view-dashboard' : 'view-dashboard-outline'} 
      size={props.size} 
      color={props.color} 
    />
  ),
  patients: (props: { focused: boolean; color: string; size: number }) => (
    <MaterialCommunityIcons 
      name={props.focused ? 'account-group' : 'account-group-outline'} 
      size={props.size} 
      color={props.color} 
    />
  ),
  appointments: (props: { focused: boolean; color: string; size: number }) => (
    <MaterialCommunityIcons 
      name={props.focused ? 'calendar-clock' : 'calendar-clock-outline'} 
      size={props.size} 
      color={props.color} 
    />
  ),
  records: (props: { focused: boolean; color: string; size: number }) => (
    <MaterialCommunityIcons 
      name={props.focused ? 'file-document' : 'file-document-outline'} 
      size={props.size} 
      color={props.color} 
    />
  ),
  alerts: (props: { focused: boolean; color: string; size: number }) => (
    <MaterialCommunityIcons 
      name={props.focused ? 'bell' : 'bell-outline'} 
      size={props.size} 
      color={props.color} 
    />
  ),
  settings: (props: { focused: boolean; color: string; size: number }) => (
    <MaterialCommunityIcons 
      name={props.focused ? 'cog' : 'cog-outline'} 
      size={props.size} 
      color={props.color} 
    />
  ),
  profile: (props: { focused: boolean; color: string; size: number }) => (
    <MaterialCommunityIcons 
      name={props.focused ? 'account' : 'account-outline'} 
      size={props.size} 
      color={props.color} 
    />
  ),
  reports: (props: { focused: boolean; color: string; size: number }) => (
    <MaterialCommunityIcons 
      name={props.focused ? 'chart-line' : 'chart-line-variant'} 
      size={props.size} 
      color={props.color} 
    />
  ),
};
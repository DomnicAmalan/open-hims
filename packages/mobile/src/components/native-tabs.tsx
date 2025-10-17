import React from 'react';
import { StyleSheet, ViewStyle, TouchableOpacity } from 'react-native';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { MaterialCommunityIcons } from '@expo/vector-icons';
import { GlassView } from 'expo-glass-effect';
import { Text } from 'react-native-paper';

// Re-export React Navigation for convenience
export { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
export { NavigationContainer } from '@react-navigation/native';
export { createNativeStackNavigator } from '@react-navigation/native-stack';
export { createDrawerNavigator } from '@react-navigation/drawer';

const Tab = createBottomTabNavigator();

// Custom Tab Bar with Glass Effect

interface GlassTabBarProps {
  state: any;
  descriptors: any;
  navigation: any;
  style?: ViewStyle;
  glassStyle?: 'clear' | 'regular';
  tintColor?: string;
}

export function GlassTabBar({ 
  state, 
  descriptors, 
  navigation, 
  style, 
  glassStyle = 'regular',
  tintColor = 'rgba(0, 102, 204, 0.1)'
}: GlassTabBarProps) {
  return (
    <GlassView 
      style={[styles.glassTabBar, style]}
      glassEffectStyle={glassStyle}
      tintColor={tintColor}
    >
      {state.routes.map((route: any, index: number) => {
        const { options } = descriptors[route.key];
        const label = options.tabBarLabel !== undefined 
          ? options.tabBarLabel 
          : options.title !== undefined 
          ? options.title 
          : route.name;

        const isFocused = state.index === index;

        const onPress = () => {
          const event = navigation.emit({
            type: 'tabPress',
            target: route.key,
            canPreventDefault: true,
          });

          if (!isFocused && !event.defaultPrevented) {
            navigation.navigate(route.name);
          }
        };

        const onLongPress = () => {
          navigation.emit({
            type: 'tabLongPress',
            target: route.key,
          });
        };

        const iconName = options.tabBarIcon?.({ 
          focused: isFocused, 
          color: isFocused ? '#0066cc' : '#666', 
          size: 24 
        });

        return (
          <TouchableOpacity
            key={route.key}
            accessibilityRole="button"
            accessibilityState={isFocused ? { selected: true } : {}}
            accessibilityLabel={options.tabBarAccessibilityLabel}
            testID={options.tabBarTestID}
            onPress={onPress}
            onLongPress={onLongPress}
            style={[
              styles.glassTabItem,
              isFocused && styles.glassTabItemActive
            ]}
          >
            <GlassView 
              style={styles.glassTabItemContainer}
              glassEffectStyle={isFocused ? 'clear' : 'regular'}
              tintColor={isFocused ? 'rgba(0, 102, 204, 0.2)' : 'rgba(255, 255, 255, 0.1)'}
              isInteractive
            >
              {iconName}
              <Text style={[
                styles.glassTabItemLabel,
                isFocused && styles.glassTabItemLabelActive
              ]}>
                {label}
              </Text>
            </GlassView>
          </TouchableOpacity>
        );
      })}
    </GlassView>
  );
}

// Healthcare Tab Navigator with Glass Effect

interface HimsTabNavigatorProps {
  children: React.ReactNode;
  screenOptions?: any;
  initialRouteName?: string;
}

export function HimsTabNavigator({ 
  children, 
  screenOptions = {},
  initialRouteName 
}: HimsTabNavigatorProps) {
  return (
    <Tab.Navigator
      initialRouteName={initialRouteName}
      screenOptions={{
        headerShown: false,
        tabBarHideOnKeyboard: true,
        tabBarStyle: { display: 'none' }, // Hide default tab bar
        ...screenOptions
      }}
      tabBar={(props) => <GlassTabBar {...props} />}
    >
      {children}
    </Tab.Navigator>
  );
}

// Preset Tab Icons for Healthcare

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

// Tab Screen Component Helper

interface HimsTabScreenProps {
  name: string;
  component: React.ComponentType<any>;
  options?: {
    tabBarLabel?: string;
    tabBarIcon?: (props: { focused: boolean; color: string; size: number }) => React.ReactNode;
    tabBarBadge?: string | number;
  };
}

export function HimsTabScreen({ name, component, options }: HimsTabScreenProps) {
  return <Tab.Screen name={name} component={component} options={options} />;
}

const styles = StyleSheet.create({
  glassTabBar: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    alignItems: 'center',
    paddingVertical: 8,
    paddingHorizontal: 16,
    borderRadius: 25,
    margin: 16,
    marginBottom: 32,
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
  },
  glassTabItem: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  glassTabItemContainer: {
    alignItems: 'center',
    justifyContent: 'center',
    paddingVertical: 8,
    paddingHorizontal: 12,
    borderRadius: 16,
    minWidth: 60,
  },
  glassTabItemActive: {
    transform: [{ scale: 1.05 }],
  },
  glassTabItemLabel: {
    fontSize: 11,
    fontWeight: '500',
    marginTop: 4,
    color: '#666',
    textAlign: 'center',
  },
  glassTabItemLabelActive: {
    color: '#0066cc',
    fontWeight: '600',
  },
});
import React from 'react';
import { StyleSheet, ViewStyle, TextStyle } from 'react-native';
import { GlassView, GlassContainer, isLiquidGlassAvailable } from 'expo-glass-effect';
import { Text as PaperText } from 'react-native-paper';

export { GlassView, GlassContainer, isLiquidGlassAvailable } from 'expo-glass-effect';

// Glass Effect Styles Type
export type GlassStyle = 'clear' | 'regular';

// Custom Glass Components for HIMS

interface HimsGlassCardProps {
  children: React.ReactNode;
  style?: ViewStyle;
  glassStyle?: GlassStyle;
  tintColor?: string;
  isInteractive?: boolean;
}

export function HimsGlassCard({ 
  children, 
  style, 
  glassStyle = 'regular', 
  tintColor = 'rgba(232, 244, 248, 0.3)',
  isInteractive = false 
}: HimsGlassCardProps) {
  return (
    <GlassView 
      style={[styles.card, style]}
      glassEffectStyle={glassStyle}
      tintColor={tintColor}
      isInteractive={isInteractive}
    >
      {children}
    </GlassView>
  );
}

interface HimsGlassHeaderProps {
  title: string;
  subtitle?: string;
  style?: ViewStyle;
  titleStyle?: TextStyle;
  subtitleStyle?: TextStyle;
}

export function HimsGlassHeader({ 
  title, 
  subtitle, 
  style, 
  titleStyle, 
  subtitleStyle 
}: HimsGlassHeaderProps) {
  return (
    <GlassView 
      style={[styles.header, style]}
      glassEffectStyle="clear"
      tintColor="rgba(0, 102, 204, 0.1)"
    >
      <PaperText variant="headlineMedium" style={[styles.headerTitle, titleStyle]}>
        {title}
      </PaperText>
      {subtitle && (
        <PaperText variant="bodyMedium" style={[styles.headerSubtitle, subtitleStyle]}>
          {subtitle}
        </PaperText>
      )}
    </GlassView>
  );
}

interface HimsGlassActionBarProps {
  children: React.ReactNode;
  style?: ViewStyle;
  spacing?: number;
}

export function HimsGlassActionBar({ 
  children, 
  style, 
  spacing = 10 
}: HimsGlassActionBarProps) {
  return (
    <GlassContainer spacing={spacing} style={[styles.actionBar, style]}>
      {children}
    </GlassContainer>
  );
}

interface HimsGlassInfoPanelProps {
  children: React.ReactNode;
  style?: ViewStyle;
  icon?: string;
  priority?: 'low' | 'medium' | 'high';
}

export function HimsGlassInfoPanel({ 
  children, 
  style, 
  priority = 'medium' 
}: HimsGlassInfoPanelProps) {
  const tintColor = priority === 'high' 
    ? 'rgba(244, 67, 54, 0.1)' 
    : priority === 'medium' 
    ? 'rgba(255, 193, 7, 0.1)' 
    : 'rgba(76, 175, 80, 0.1)';

  return (
    <GlassView 
      style={[styles.infoPanel, style]}
      glassEffectStyle="regular"
      tintColor={tintColor}
      isInteractive
    >
      {children}
    </GlassView>
  );
}

const styles = StyleSheet.create({
  card: {
    padding: 16,
    borderRadius: 12,
    margin: 8,
  },
  header: {
    padding: 20,
    borderRadius: 16,
    alignItems: 'center',
    marginBottom: 16,
  },
  headerTitle: {
    fontWeight: 'bold',
    textAlign: 'center',
    marginBottom: 4,
  },
  headerSubtitle: {
    textAlign: 'center',
    opacity: 0.8,
  },
  actionBar: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    alignItems: 'center',
    padding: 12,
    borderRadius: 20,
    margin: 8,
  },
  infoPanel: {
    padding: 16,
    borderRadius: 8,
    margin: 4,
    borderLeftWidth: 4,
    borderLeftColor: 'rgba(0, 102, 204, 0.5)',
  },
});
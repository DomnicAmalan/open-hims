import { Card, Text, Group } from '@mantine/core';
import type { CardProps } from '@mantine/core';

interface TestCardProps extends CardProps {
  title?: string;
  content?: string;
}

export function TestCard({ title = 'Test Card', content = 'This is a test card component', ...props }: TestCardProps) {
  return (
    <Card shadow="sm" padding="lg" radius="md" withBorder {...props}>
      <Group justify="space-between" mb="xs">
        <Text fw={500}>{title}</Text>
      </Group>
      <Text size="sm" c="dimmed">
        {content}
      </Text>
    </Card>
  );
}
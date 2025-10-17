import { Button } from '@mantine/core';
import type { ButtonProps } from '@mantine/core';

interface TestButtonProps extends ButtonProps {
  label?: string;
}

export function TestButton({ label = 'Test Button', ...props }: TestButtonProps) {
  return (
    <Button {...props}>
      {label}
    </Button>
  );
}
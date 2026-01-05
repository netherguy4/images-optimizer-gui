export const useTheme = () => {
  const themeCookie = useCookie<'light' | 'dark' | undefined>(
    'theme-preference',
  );

  const theme = useState<'light' | 'dark' | undefined>(
    'theme',
    () => themeCookie.value,
  );

  useHead({
    htmlAttrs: {
      style: () => `color-scheme: ${theme.value || 'light dark'};`,
    },
  });

  onMounted(() => {
    if (!theme.value) {
      const systemDark = window.matchMedia(
        '(prefers-color-scheme: dark)',
      ).matches;
      theme.value = systemDark ? 'dark' : 'light';
    }
  });

  const toggleTheme = () => {
    const newTheme = theme.value === 'dark' ? 'light' : 'dark';

    theme.value = newTheme;
    themeCookie.value = newTheme;
  };

  return { theme, toggleTheme };
};

import { FC, useEffect } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormMessage,
  Form,
} from "@/components/ui/field";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import {
  getAvailableThemes,
  getCurrentTheme,
  applyTheme,
  selectThemes,
  selectCurrentTheme,
  Theme,
} from "@/store/settingsSlice";
import { Check, ChevronsUpDown } from "lucide-react";
import { cn } from "@/lib/utils";

const themeSettingsSchema = z.object({
  theme: z.object({
    name: z.string(),
    path: z.string(),
  }),
});

export const ThemeSettings: FC = () => {
  const dispatch = useAppDispatch();
  const themes = useAppSelector(selectThemes);
  const currentTheme = useAppSelector(selectCurrentTheme);

  const form = useForm<z.infer<typeof themeSettingsSchema>>({
    resolver: zodResolver(themeSettingsSchema),
  });

  useEffect(() => {
    dispatch(getAvailableThemes());
    dispatch(getCurrentTheme());
  }, [dispatch]);

  useEffect(() => {
    if (currentTheme) {
      form.reset({ theme: currentTheme });
    }
  }, [currentTheme, form]);

  const onSubmit = (values: z.infer<typeof themeSettingsSchema>) => {
    dispatch(applyTheme(values.theme));
  };

  const onThemeSelect = (theme: Theme) => {
    form.setValue("theme", theme);
  };

  const onCancel = () => {
    if (currentTheme) {
      form.reset({ theme: currentTheme });
    }
  };

  const onReset = () => {
    const defaultTheme = themes.find((theme) => theme.name === "default");
    if (defaultTheme) {
      form.reset({ theme: defaultTheme });
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Themes</CardTitle>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
            <FormField
              control={form.control}
              name="theme"
              render={({ field }) => (
                <FormItem className="flex flex-col">
                  <FormLabel>Theme</FormLabel>
                  <Popover>
                    <PopoverTrigger asChild>
                      <FormControl>
                        <Button
                          variant="outline"
                          role="combobox"
                          className={cn(
                            "w-[200px] justify-between",
                            !field.value && "text-muted-foreground"
                          )}
                        >
                          {field.value
                            ? themes.find(
                                (theme) => theme.name === field.value.name
                              )?.name
                            : "Select theme"}
                          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                        </Button>
                      </FormControl>
                    </PopoverTrigger>
                    <PopoverContent className="w-[200px] p-0">
                      <Command>
                        <CommandInput placeholder="Search theme..." />
                        <CommandEmpty>No theme found.</CommandEmpty>
                        <CommandGroup>
                          {themes.map((theme) => (
                            <CommandItem
                              value={theme.name}
                              key={theme.name}
                              onSelect={() => onThemeSelect(theme)}
                            >
                              <Check
                                className={cn(
                                  "mr-2 h-4 w-4",
                                  theme.name === field.value?.name
                                    ? "opacity-100"
                                    : "opacity-0"
                                )}
                              />
                              {theme.name}
                            </CommandItem>
                          ))}
                        </CommandGroup>
                      </Command>
                    </PopoverContent>
                  </Popover>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className="flex justify-end space-x-2">
              <Button type="button" variant="outline" onClick={onCancel}>
                Cancel
              </Button>
              <Button type="button" variant="outline" onClick={onReset}>
                Reset to Defaults
              </Button>
              <Button type="submit">Apply</Button>
            </div>
          </form>
        </Form>
      </CardContent>
    </Card>
  );
};

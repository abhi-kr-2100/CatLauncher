import { FC, useEffect, useState } from "react";
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
  getMonospaceFonts,
  getCurrentFont,
  applyFont,
  selectFonts,
  selectCurrentFont,
  Font,
} from "@/store/settingsSlice";
import { Check, ChevronsUpDown } from "lucide-react";
import { cn } from "@/lib/utils";
import { convertFileSrc } from "@tauri-apps/api/path";

const fontSettingsSchema = z.object({
  font: z.object({
    name: z.string(),
    path: z.string(),
  }),
});

export const FontSettings: FC = () => {
  const dispatch = useAppDispatch();
  const fonts = useAppSelector(selectFonts);
  const currentFont = useAppSelector(selectCurrentFont);
  const [fontPreviewUrl, setFontPreviewUrl] = useState<string | null>(null);

  const form = useForm<z.infer<typeof fontSettingsSchema>>({
    resolver: zodResolver(fontSettingsSchema),
  });

  useEffect(() => {
    dispatch(getMonospaceFonts());
    dispatch(getCurrentFont());
  }, [dispatch]);

  useEffect(() => {
    if (currentFont) {
      form.reset({ font: currentFont });
      const url = convertFileSrc(currentFont.path);
      setFontPreviewUrl(url);
    }
  }, [currentFont, form]);

  const onSubmit = (values: z.infer<typeof fontSettingsSchema>) => {
    dispatch(applyFont(values.font));
  };

  const onFontSelect = (font: Font) => {
    form.setValue("font", font);
    const url = convertFileSrc(font.path);
    setFontPreviewUrl(url);
  };

  const onCancel = () => {
    if (currentFont) {
      form.reset({ font: currentFont });
      const url = convertFileSrc(currentFont.path);
      setFontPreviewUrl(url);
    }
  };

  const onReset = () => {
    // There is no default font, so we do nothing.
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Fonts</CardTitle>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
            <FormField
              control={form.control}
              name="font"
              render={({ field }) => (
                <FormItem className="flex flex-col">
                  <FormLabel>Font</FormLabel>
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
                            ? fonts.find((font) => font.name === field.value.name)
                                ?.name
                            : "Select font"}
                          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                        </Button>
                      </FormControl>
                    </PopoverTrigger>
                    <PopoverContent className="w-[200px] p-0">
                      <Command>
                        <CommandInput placeholder="Search font..." />
                        <CommandEmpty>No font found.</CommandEmpty>
                        <CommandGroup>
                          {fonts.map((font) => (
                            <CommandItem
                              value={font.name}
                              key={font.name}
                              onSelect={() => onFontSelect(font)}
                            >
                              <Check
                                className={cn(
                                  "mr-2 h-4 w-4",
                                  font.name === field.value?.name
                                    ? "opacity-100"
                                    : "opacity-0"
                                )}
                              />
                              {font.name}
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
            {fontPreviewUrl && (
              <div>
                <style>
                  {`
                    @font-face {
                      font-family: 'FontPreview';
                      src: url('${fontPreviewUrl}') format('truetype');
                    }
                  `}
                </style>
                <div
                  style={{ fontFamily: "FontPreview", fontSize: "20px" }}
                >
                  The quick brown fox jumps over the lazy dog. 1234567890
                </div>
              </div>
            )}
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

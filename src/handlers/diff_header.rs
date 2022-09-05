#[derive(Debug, PartialEq, Eq)]
    Added,
    Removed,
    fn should_write_generic_diff_header_header_line(&mut self) -> std::io::Result<bool> {
        // In color_only mode, raw_line's structure shouldn't be changed.
        // So it needs to avoid fn _handle_diff_header_header_line
        // (it connects the plus_file and minus_file),
        // and to call fn handle_generic_diff_header_header_line directly.
        if self.config.color_only {
            write_generic_diff_header_header_line(
                &self.line,
                &self.raw_line,
                &mut self.painter,
                &mut self.mode_info,
                self.config,
            )?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

        self.painter.paint_buffered_minus_and_plus_lines();
        self.should_write_generic_diff_header_header_line()
        self.painter.paint_buffered_minus_and_plus_lines();
        if self.should_write_generic_diff_header_header_line()? {
            handled_line = true;
            self.handled_diff_header_header_line_file_pair = self.current_file_pair.clone();
        }
        Ok(handled_line)
    }

    #[inline]
    fn test_diff_header_file_operation_line(&self) -> bool {
        (matches!(self.state, State::DiffHeader(_)) || self.source == Source::DiffUnified)
            && (self.line.starts_with("deleted file mode ")
                || self.line.starts_with("new file mode "))
    }

    /// Check for and handle the "deleted file ..."  line.
    pub fn handle_diff_header_file_operation_line(&mut self) -> std::io::Result<bool> {
        if !self.test_diff_header_file_operation_line() {
            return Ok(false);
        }
        let mut handled_line = false;
        let (_mode_info, file_event) =
            parse_diff_header_line(&self.line, self.source == Source::GitDiff);
        let name = get_repeated_file_path_from_diff_line(&self.diff_line)
            .unwrap_or_else(|| "".to_string());
        match file_event {
            FileEvent::Removed => {
                self.minus_file = name;
                self.plus_file = "/dev/null".into();
                self.minus_file_event = FileEvent::Change;
                self.plus_file_event = FileEvent::Change;
                self.current_file_pair = Some((self.minus_file.clone(), self.plus_file.clone()));
            }
            FileEvent::Added => {
                self.minus_file = "/dev/null".into();
                self.plus_file = name;
                self.minus_file_event = FileEvent::Change;
                self.plus_file_event = FileEvent::Change;
                self.current_file_pair = Some((self.minus_file.clone(), self.plus_file.clone()));
            }
            _ => (),
        }

        if self.should_write_generic_diff_header_header_line()?
            || (self.should_handle()
                && self.handled_diff_header_header_line_file_pair != self.current_file_pair)
        {
            handled_line = true;
    #[inline]
    fn test_pending_line_with_diff_name(&self) -> bool {
        matches!(self.state, State::DiffHeader(_)) || self.source == Source::DiffUnified
    }

    pub fn handle_pending_line_with_diff_name(&mut self) -> std::io::Result<()> {
        if !self.test_pending_line_with_diff_name() {
            return Ok(());
        }

        } else if !self.config.color_only
            && self.should_handle()
            && self.handled_diff_header_header_line_file_pair != self.current_file_pair
        {
            self._handle_diff_header_header_line(self.source == Source::DiffUnified)?;
            self.handled_diff_header_header_line_file_pair = self.current_file_pair.clone();
            Ok(())
        line if line.starts_with("new file mode ") => {
            (line[14..].to_string(), FileEvent::Added) // "new file mode ".len()
        }
        line if line.starts_with("deleted file mode ") => {
            (line[18..].to_string(), FileEvent::Removed) // "deleted file mode ".len()
        }
        let format_file = |file| {
            let formatted_file = if let Some(regex_replacement) = &config.file_regex_replacement {
                regex_replacement.execute(file)
            } else {
                Cow::from(file)
            };
            match (config.hyperlinks, utils::path::absolute_path(file, config)) {
                (true, Some(absolute_path)) => features::hyperlinks::format_osc8_file_hyperlink(
                    absolute_path,
                    None,
                    &formatted_file,
                    config,
                ),
                _ => formatted_file,
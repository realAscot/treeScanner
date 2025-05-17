#[cfg(test)]
mod tests {
    use treescanner::app::treebuilder::{TreeBuilder, TreeBuilderConfig};
    use std::path::PathBuf;

    fn mock_tree_builder() -> TreeBuilder {
        let config = TreeBuilderConfig {
            root_path: PathBuf::from("/mock"),
            max_depth: Some(1), 
            max_files_per_dir: 3,
            ignored_dirs: vec![],
            folder_icon: "📁".to_string(),
            file_icon: "📄".to_string(),
            align_comments: true,
        };
        TreeBuilder::new(config)
    }

    #[test]
    fn test_align_lines_with_comments() {
        let builder = mock_tree_builder();
        let lines = vec![
            "📁 src/".to_string(),
            "├── 📄 main.rs".to_string(),
            "└── 📄 lib.rs".to_string(),
        ];

        let aligned = builder.align_lines_with_comments(&lines);

        // Prüfe, ob jede Zeile mit einem # endet
        for line in aligned {
            assert!(line.trim_end().ends_with('#'), "Fehlendes #: {}", line);
        }
    }
}
